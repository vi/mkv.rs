use super::EventsHandler;
use super::Parser;
use super::Info;
use super::database::{id_to_class,class_to_type};

use self::parse_ebml_number::parse_ebml_number;
use self::parse_ebml_number::Result as EbmlParseNumberResult;
use self::parse_ebml_number::Mode   as EbmlParseNumberMode;

mod parse_ebml_number;

pub enum ParserMode {
    Header,
    Data(usize),
    Resync,
}


pub struct ParserState<E> {
    cb : E,
    accumulator : Vec<u8>,
    opened_elements_stack : Vec<Info>,
    mode : ParserMode,
    current_offset : u64,
}

#[derive(Debug,Eq,PartialEq)]
enum ResultOfTryParseSomething<'a> {
    KeepGoing(&'a [u8]),
    NoMoreData,
    Error,
}

impl<E:EventsHandler> ParserState<E> {

    fn try_resync<'a>(&mut self, buf:&'a [u8]) -> ResultOfTryParseSomething<'a> {
        use self::ResultOfTryParseSomething::*;
        if buf.len() < 4 {
            return NoMoreData;
        }
        let (fourbytes, _) = buf.split_at(4);
        let id : u32 = (fourbytes[0] as u32)*0x1000000 + (fourbytes[1] as u32) * 0x10000 + (fourbytes[2] as u32) * 0x100 + (fourbytes[3] as u32);
        match id {
            0x1A45DFA3 | 0x18538067 | 0x1549A966 | 0x1F43B675 | 0x1654AE6B | 0x1C53BB6B | 0x1941A469 | 0x1043A770 | 0x1254C367
            => { 
                self.mode = ParserMode::Header; 
                KeepGoing(buf) 
            }
            _ => {
                let (_, trail) = buf.split_at(1);
                KeepGoing(trail)
            }
        }
    }
    
    fn try_parse_element_data<'a>(&mut self, buf:&'a [u8], len:usize) -> ResultOfTryParseSomething<'a> {
        use self::ResultOfTryParseSomething::{NoMoreData,KeepGoing};
        use super::Event::{Data};
        use super::SimpleContent::
                    {Unsigned, Signed, Text, Binary, Float, Date_NanosecondsSince20010101_000000_UTC};
        use self::ParserMode;
        
        if len <= buf.len() {
            self.mode = ParserMode::Header;
            let (l,r) = buf.split_at(len);
            self.cb.event( Data( Binary(l) ));
            KeepGoing(r)
        } else {
            return NoMoreData;
        }
    }
    
    fn try_parse_element_header<'a>(&mut self, buf:&'a [u8]) -> ResultOfTryParseSomething<'a> {
        use self::ResultOfTryParseSomething::{NoMoreData,KeepGoing};
        use self::ResultOfTryParseSomething::Error as MyError;
        use self::parse_ebml_number::Result::*;
        use self::parse_ebml_number::Mode::*;
        use super::Type::{Master};
        use super::Event::{Begin};
        use self::ParserMode;
        
        let (r1, restbuf) = parse_ebml_number(buf, Identifier);
        let element_id = match r1 {
            Error => return MyError,
            NaN => return MyError,
            NotEnoughData => return NoMoreData,
            Ok(x) => x
        };
        
        let (r2, restbuf2) = parse_ebml_number(restbuf, Unsigned);
        
        let element_header_size = (buf.len() - restbuf2.len()) as u64;
        let element_size = match r2 {
            Error => return MyError,
            NaN => None,
            NotEnoughData => return NoMoreData,
            Ok(x) => Some(x)
        };
        let full_element_size = match element_size {
            None => None,
            Some(x) => Some(x + element_header_size),
        };
        
        let cl  = id_to_class(element_id);
        let typ = class_to_type(cl);
        
                                                        
        let mut restbuf3 = restbuf2;
        
        self.mode = match typ {
            Master => ParserMode::Header,
            _ => match element_size {
                None => ParserMode::Resync,
                Some(x) => ParserMode::Data(x as usize),
            }
        };
        
        let el = Info{id: element_id, length_including_header: full_element_size, offset: self.current_offset}; 
        self.cb.event( Begin( &el ));
        self.opened_elements_stack.push(el);
        //self.cb.auxilary_event( Debug (format!("element class={:?} type={:?} off={} clid={}  len={:?}",
        //                                                cl, typ, self.current_offset, element_id, element_size )));
                                                        
        KeepGoing(restbuf3)
    }
    
    fn close_expired_elements<'a>(&mut self) {
        use super::Event::{End};
        let mut number_of_elements_to_remove = 0;
        
        for i in self.opened_elements_stack.iter().rev() {
            let retain = match i.length_including_header {
                None => true,
                Some(l) => i.offset + l > self.current_offset
            };
            //self.cb.log (format!("dr {:?} {} -> {}", i, self.current_offset, retain).as_str());
            
            if retain {
                break;
            } else {
                number_of_elements_to_remove += 1;
            }
        }
        
        {
            let mut j = 0;
            for i in self.opened_elements_stack.iter().rev() {
                j += 1;
                if j > number_of_elements_to_remove { break; }
                self.cb.event (End(i));
            }
        }
        
        let newlen = self.opened_elements_stack.len() - number_of_elements_to_remove;
        self.opened_elements_stack.truncate(newlen);
    }
}


impl<E:EventsHandler> Parser<E> for ParserState<E> {
    fn initialize(cb : E) -> ParserState<E> {
        ParserState {
            accumulator: vec![],
            cb : cb,
            mode : ParserMode::Header,
            opened_elements_stack : vec![],
            current_offset : 0,
        }
    }
    
    fn feed_bytes(&mut self, bytes : &[u8])
    {
        use self::ResultOfTryParseSomething::*;
        use self::ParserMode::*;
        
        //self.cb.log(format!("feed_bytes {} len={}", bytes[0], self.accumulator.len()).as_str() );
        self.accumulator.push_all(bytes);
        
        let tmpvector = self.accumulator.to_vec();
        {
            let mut buf = tmpvector.as_slice();
            //self.cb.log( format!("feed_bytes2 len={} buflen={}", self.accumulator.len(), buf.len()) );
            loop {
                let r = match self.mode {
                    Resync => self.try_resync(buf),
                    Header => self.try_parse_element_header(buf),
                    Data(x) => self.try_parse_element_data(buf, x),
                };
                //self.cb.log( format!("try_parse_something={:?}", r));
                let newbuf = match r {
                    NoMoreData => break,
                    Error => {self.mode = ParserMode::Resync; self.cb.event(super::Event::Resync); buf},
                    KeepGoing(rest) => rest
                };
                self.current_offset += (buf.len() - newbuf.len()) as u64;
                //self.cb.log(format!("current offset: {}", self.current_offset).as_str());
                self.close_expired_elements();
                buf = newbuf;
                //self.cb.log(format!("more to parse: {}", newbuf.len()).as_str());
            }
            self.accumulator = buf.to_vec();
        }
        //self.cb.log( format!("feed_bytes3 len={}" , self.accumulator.len()).as_str());
    }
}
