use std::vec::Vec;
use std::string::String;
use std::fmt;

pub mod parser;
pub mod database;


#[derive(Debug,Eq,PartialEq,Clone,Copy)]
pub enum Type {
    Master,
    Unsigned,
    Signed,
    TextAscii,
    TextUtf8,
    Binary,
    Float,
    Date,

}
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
    fn initialize(cb : E) -> Self;
    fn feed_bytes(&mut self, bytes : &[u8]);
    fn force_resync(&mut self);
}

impl fmt::Debug for Info {
    fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result
    {
        let cl = database::id_to_class(self.id);
        let typ = database::class_to_type(cl);
        
        let cldesc = match cl {
            database::Class::Unknown => format!("0x{:X}", self.id),
            _ => format!("{:?}", cl),
        };
        
        let maybelen = match self.length_including_header {
            None => format!(""),
            Some(x) => format!(", rawlen:{}", x),
        };
        
        f.write_str(format!("{}(offset:{}{})", cldesc, self.offset, maybelen).as_slice())
    }
}
