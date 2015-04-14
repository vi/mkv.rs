use std::rc::Rc;
use std::vec::Vec;

pub mod database;
pub mod parser;  // bytes -> events
pub mod generator; // DOM -> bytes

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


// Element DOM

#[derive(PartialEq,Debug,PartialOrd,Clone)]
pub enum ElementContent {
    Master(Vec<Rc<Element>>),
    Unsigned(u64),
    Signed(i64),
    Binary(Rc<Vec<u8>>),
    Text(Rc<String>),
    Float(f64),
    Date_NanosecondsSince20010101_000000_UTC(i64),
    Unknown(u64, Rc<Vec<u8>>)
}

#[derive(PartialEq,Debug,PartialOrd,Clone)]
pub struct Element {
    class : database::Class,
    content : ElementContent,
}
