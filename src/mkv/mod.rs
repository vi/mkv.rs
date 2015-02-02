use std::vec::Vec;
use std::string::String;

pub mod parser;
pub mod elements;

pub enum AuxilaryEvent {
    Debug(String),
    Warning(String),
}

pub trait EventsHandler {
    fn auxilary_event(&mut self, e : AuxilaryEvent);
}

pub trait Parser<E : EventsHandler> {
    fn initialize(cb : E) -> Self;
    fn feed_bytes(&mut self, bytes : &[u8]);
}
