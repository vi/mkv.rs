use std::vec::Vec;
use std::string::String;
use std::fmt;

pub mod parser;
pub mod elements_database;

#[derive(Debug,Eq,PartialEq)]
pub enum ElementType {
    Master,
    Unsigned,
    Signed,
    TextAscii,
    TextUtf8,
    Binary,
    Float,
    Date,
}

pub struct ElementInfo {
    id : u64,
    offset : u64,
    length_including_header : Option<u64>,
}

#[derive(Debug,PartialEq)]
pub enum SimpleElementContent<'a> {
    Unsigned(u64),
    Signed(i64),
    Text(&'a str),
    Binary(&'a [u8]),
    Float(f64),
    Date_NanosecondsSince20010101_000000_UTC(i64),
}

pub enum ElementEvent<'a> {
    ElementBegin(&'a ElementInfo),
    ElementData(SimpleElementContent<'a>),
    ElementEnd(&'a ElementInfo),
    Resync,
}

pub trait ElementEventsHandler {
    fn event(&mut self, e : ElementEvent);
    fn log(&mut self, m : &str);
}

pub trait ElementParser<E : ElementEventsHandler > {
    fn initialize(cb : E) -> Self;
    fn feed_bytes(&mut self, bytes : &[u8]);
}

impl fmt::Debug for ElementInfo {
    fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result
    {
        let cl = elements_database::id_to_class(self.id);
        let typ = elements_database::class_to_type(cl);
        
        let cldesc = match cl {
            elements_database::Class::Unknown => format!("0x{:X}", self.id),
            _ => format!("{:?}", cl),
        };
        
        let maybelen = match self.length_including_header {
            None => format!(""),
            Some(x) => format!(", rawlen:{}", x),
        };
        
        f.write_str(format!("{}(offset:{}{})", cldesc, self.offset, maybelen).as_slice())
    }
}
