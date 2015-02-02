use std::vec::Vec;
use std::string::String;
use std::fmt;

pub mod parser;
pub mod elements;


pub struct ElementInfo {
    id : u64,
    offset : u64,
    length : Option<u64>,
}

pub enum AuxilaryEvent<'a> {
    Debug(String),
    Warning(String),
    ElementBegin(ElementInfo),
    ElementData(&'a [u8]),
}

pub trait EventsHandler {
    fn auxilary_event(&mut self, e : AuxilaryEvent);
}

pub trait Parser<E : EventsHandler> {
    fn initialize(cb : E) -> Self;
    fn feed_bytes(&mut self, bytes : &[u8]);
}

impl fmt::Show for ElementInfo {
    fn fmt(&self, f:&mut fmt::Formatter) -> fmt::Result
    {
        let cl = elements::id_to_class(self.id);
        let typ = elements::class_to_type(&cl);
        
        let cldesc = match cl {
            elements::Class::Unknown => format!("0x{:X}", self.id),
            _ => format!("{:?}", cl),
        };
        
        let maybelen = match self.length {
            None => format!(""),
            Some(x) => format!(", length:{}", x),
        };
        
        f.write_str(format!("{}(offset:{}{})", cldesc, self.offset, maybelen).as_slice())
    }
}
