use mkv::EventsHandler;
use mkv::Parser;
use mkv::ElementInfo;
use mkv::elements::{id_to_class,class_to_type};

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
    opened_elements_stack : Vec<ElementInfo>,
    mode : ParserMode,
    current_offset : u64,
}

#[derive(Show,Eq,PartialEq)]
enum ResultOfTryParseSomething<'a> {
    KeepGoing(&'a [u8]),
    NoMoreData,
    Error,
}

impl<E:EventsHandler> ParserState<E> {
    fn try_parse_something<'a>(&mut self, buf:&'a [u8]) -> ResultOfTryParseSomething<'a> {
        use self::ParserMode::*;
        match self.mode {
            Resync => panic!("Resyncing is not implemented yet"),
            Header => self.try_parse_element_header(buf),
            Data(x) => self.try_parse_element_data(buf, x),
        }
    }
    
    fn try_parse_element_data<'a>(&mut self, buf:&'a [u8], len:usize) -> ResultOfTryParseSomething<'a> {
        use self::ResultOfTryParseSomething::{NoMoreData,KeepGoing};
        use mkv::AuxilaryEvent::{ElementData};
        use mkv::SimpleElementContent::
                    {Unsigned, Signed, Text, Binary, Float, Date_NanosecondsSince20010101_000000_UTC};
        use self::ParserMode;
        
        if len < buf.len() {
            self.mode = ParserMode::Header;
            let (l,r) = buf.split_at(len);
            self.cb.auxilary_event( ElementData( Binary(l) ));
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
        use mkv::elements::Type::{Master};
        use mkv::AuxilaryEvent::{Debug,ElementBegin};
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
        let typ = class_to_type(&cl);
        
                                                        
        let mut restbuf3 = restbuf2;
        
        self.mode = match typ {
            Master => ParserMode::Header,
            _ => match element_size {
                None => ParserMode::Resync,
                Some(x) => ParserMode::Data(x as usize),
            }
        };
        
        let el = ElementInfo{id: element_id, length_including_header: full_element_size, offset: self.current_offset}; 
        self.cb.auxilary_event( ElementBegin( &el ));
        self.opened_elements_stack.push(el);
        //self.cb.auxilary_event( Debug (format!("element class={:?} type={:?} off={} clid={}  len={:?}",
        //                                                cl, typ, self.current_offset, element_id, element_size )));
                                                        
        KeepGoing(restbuf3)
    }
    
    fn close_expired_elements<'a>(&mut self) {
        use mkv::AuxilaryEvent::{Debug,ElementEnd};
        let mut number_of_elements_to_remove = 0;
        
        for i in self.opened_elements_stack.iter().rev() {
            let retain = match i.length_including_header {
                None => true,
                Some(l) => i.offset + l > self.current_offset
            };
            //self.cb.auxilary_event (Debug(format!("dr {:?} {} -> {}", i, self.current_offset, retain).as_slice()));
            
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
                self.cb.auxilary_event (ElementEnd(i));
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
        
        //self.cb.auxilary_event( Debug (format!("feed_bytes {} len={}", bytes[0], self.accumulator.len()) ));
        self.accumulator.push_all(bytes);
        
        let tmpvector = self.accumulator.to_vec();
        {
            let mut buf = tmpvector.as_slice();
            //self.cb.auxilary_event( Debug (format!("feed_bytes2 len={} buflen={}", self.accumulator.len(), buf.len()) ));
            loop {
                let r = self.try_parse_something(buf);
                //self.cb.auxilary_event( Debug (format!("try_parse_something={:?}", r)));
                let newbuf = match r {
                    NoMoreData => break,
                    Error => panic!("Need to implement resyncing"),
                    KeepGoing(rest) => rest
                };
                self.current_offset += (buf.len() - newbuf.len()) as u64;
                self.close_expired_elements();
                buf = newbuf;
            }
            self.accumulator = buf.to_vec();
        }
        //self.cb.auxilary_event( Debug (format!("feed_bytes3 len={}" , self.accumulator.len())));
    }
}
