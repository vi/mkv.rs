use mkv::EventsHandler;
use mkv::Parser;
use mkv::AuxilaryEvent::Debug;
use mkv::elements::{id_to_class,class_to_type};

use self::parse_ebml_number::parse_ebml_number;
use self::parse_ebml_number::Result as EbmlParseNumberResult;
use self::parse_ebml_number::Mode   as EbmlParseNumberMode;

mod parse_ebml_number;


pub struct OpenedElement {
    id : u64,
    offset : u64,
    length : Option<u64>,
}

pub struct ParserState<E> {
    cb : E,
    accumulator : Vec<u8>,
    opened_elements_stack : Vec<OpenedElement>,
    resyncing : bool,
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
        use self::ResultOfTryParseSomething::{NoMoreData,KeepGoing};
        use self::ResultOfTryParseSomething::Error as MyError;
        use self::parse_ebml_number::Result::*;
        use self::parse_ebml_number::Mode::*;
        use mkv::elements::Type::{Master};
        
        let (r1, restbuf) = parse_ebml_number(buf, Identifier);
        let element_id = match r1 {
            Error => return MyError,
            NaN => return MyError,
            NotEnoughData => return NoMoreData,
            Ok(x) => x
        };
        
        let (r2, restbuf2) = parse_ebml_number(restbuf, Unsigned);
        let element_size = match r2 {
            Error => return MyError,
            NaN => None,
            NotEnoughData => return NoMoreData,
            Ok(x) => Some(x)
        };
        
        let cl  = id_to_class(element_id);
        let typ = class_to_type(&cl);
        
                                                        
        let mut restbuf3 = restbuf2;
        
        match typ {
            Master => (),
            _ => match element_size {
                None => (),
                Some(x) => {
                            let datalen = x as usize;
                            if datalen < restbuf2.len() {
                                let (l,r) = restbuf2.split_at(datalen); restbuf3 = r;
                            } else {
                                return NoMoreData;
                            }
                           } 
            }
        }
        
        self.cb.auxilary_event( Debug (format!("element class={:?} type={:?} off={} clid={}  len={:?}",
                                                        cl, typ, self.current_offset, element_id, element_size )));
                                                        
        KeepGoing(restbuf3)
    }
}


impl<E:EventsHandler> Parser<E> for ParserState<E> {
    fn initialize(cb : E) -> ParserState<E> {
        ParserState {
            accumulator: vec![],
            cb : cb,
            resyncing : false,
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
                buf = newbuf;
            }
            self.accumulator = buf.to_vec();
        }
        //self.cb.auxilary_event( Debug (format!("feed_bytes3 len={}" , self.accumulator.len())));
    }
}
