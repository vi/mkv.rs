use super::database::{id_to_class,class_to_type};

use self::parse_ebml_number::parse_ebml_number;
use self::parse_ebml_number::Result as EbmlParseNumberResult;
use self::parse_ebml_number::Mode   as EbmlParseNumberMode;

use std::vec::Vec;
use std::string::String;
use std::fmt;


mod parse_ebml_number;
mod test;
pub mod debug;

extern crate byteorder;


#[derive(Eq,PartialEq,Clone)]
pub struct Info {
    id : u64,
    offset : u64,
    length_including_header : Option<u64>,
}

#[derive(Debug,PartialEq,Clone)]
pub enum SimpleContent<'a> {
    Unsigned(u64),
    Signed(i64),
    Text(&'a str),
    Binary(&'a [u8]),
    Float(f64),
    Date_NanosecondsSince20010101_000000_UTC(i64),

}
#[derive(Debug,PartialEq,Clone)]
pub enum Event<'a> {
    Begin(&'a Info),
    Data(SimpleContent<'a>),
    End(&'a Info),
    Resync,
}

pub trait EventsHandler {
    fn event(&mut self, e : Event);
    fn log(&mut self, m : &str);
}

pub trait Parser<E : EventsHandler > {
    fn new(cb : E) -> Self;
    fn feed_bytes(&mut self, bytes : &[u8]);
    fn force_resync(&mut self);
}

pub fn new<E : EventsHandler> (cb: E) -> ParserState<E> {
    self::Parser::new(cb)
}

//////////////////////////////

impl fmt::Debug for Info {
    fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result
    {
        let cl = super::database::id_to_class(self.id);
        let typ = super::database::class_to_type(cl);
        
        let cldesc = match cl {
            super::database::Class::Unknown => format!("0x{:X}", self.id),
            _ => format!("{:?}", cl),
        };
        
        let maybelen = match self.length_including_header {
            None => format!(""),
            Some(x) => format!(", rawlen:{}", x),
        };
        
        f.write_str(format!("{}(offset:{}{})", cldesc, self.offset, maybelen).as_str())
    }
}


enum ParserMode {
    Header,
    Data(usize, super::Type),
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
    
    fn try_parse_element_data<'a>(&mut self, buf:&'a [u8], len:usize, typ : super::Type) -> ResultOfTryParseSomething<'a> {
        use self::ResultOfTryParseSomething::{NoMoreData,KeepGoing,Error};
        use self::Event::{Data};
        use self::SimpleContent::
                    {Unsigned, Signed, Text, Binary, Float, Date_NanosecondsSince20010101_000000_UTC};
        use self::ParserMode;
        
        if len <= buf.len() {
            self.mode = ParserMode::Header;
            let (l,r) = buf.split_at(len);
            let da = match typ {
                super::Type::Master => panic!("Wrong way"),
                super::Type::Binary => Binary(l),
                super::Type::Unsigned => {
                    let mut q : u64 = 0;
                    let mut it = l.iter();
                    loop {
                        match it.next() {
                            Some(x) => {q = q*0x100 + (*x as u64)},
                            None => break,
                        }
                    };
                    Unsigned(q)
                },
                super::Type::Signed | super::Type::Date => {
                    let mut q : i64 = 0;
                    let mut it = l.iter();
                    match it.next() {
                        Some(&x) if x >= 0x80 =>   { q = -(x as i64) + 0x80; },
                        Some(&x)              =>   { q = (x as i64); },
                        None => {},
                    }
                    loop {
                        match it.next() {
                            Some(&x) => {q = q*0x100 + (x as i64)},
                            None => break,
                        }
                    };
                    match typ {
                        super::Type::Signed => Signed(q),
                        super::Type::Date => Date_NanosecondsSince20010101_000000_UTC(q),
                        _ => panic!("Internal error"),
                    }
                },
                super::Type::TextAscii | super::Type::TextUtf8 => {
                    use std::str::from_utf8;
                    match from_utf8(l) {
                        Ok(x) => Text(x),
                        Err(_) => return Error,
                    }
                },
                super::Type::Float => {
                    use self::byteorder::ReadBytesExt;
                    use self::byteorder::BigEndian;
                    let mut ll = l;
                    match ll.len() {
                        8 => 
                            match ll.read_f64::<BigEndian>() {
                                Ok(x) => Float(x),
                                Err(_) => return Error,
                            },
                        4 => 
                            match ll.read_f32::<BigEndian>() {
                                Ok(x) => Float(x as f64),
                                Err(_) => return Error,
                            },
                        0 => Float(0.0),
                        10 => { self.cb.log("Error: 10-byte floats are not supported"); Binary(l) }
                        _ => return Error,
                    }
                }
            };
            self.cb.event( Data(da) );
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
        use self::Event::{Begin};
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
            t => match element_size {
                None => ParserMode::Resync,
                Some(x) => ParserMode::Data(x as usize, t),
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
        use self::Event::{End};
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
    fn new(cb : E) -> ParserState<E> {
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
                    Data(x, t) => self.try_parse_element_data(buf, x, t),
                };
                //self.cb.log( format!("try_parse_something={:?}", r));
                let newbuf = match r {
                    NoMoreData => break,
                    Error => {self.mode = ParserMode::Resync; self.cb.event(self::Event::Resync); buf},
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
    
    fn force_resync(&mut self)
    {
        self.mode = self::ParserMode::Resync;
    }
}
