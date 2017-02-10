#![allow(unused_imports)]
#![allow(dead_code)]

#[macro_use]
extern crate nom;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct EbmlNumber {
    pub raw_value: u64,
    pub length: u8,
}

type EbmlElementRawId = u64;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct EbmlElementHeader {
    pub id: EbmlElementRawId,
    pub len: Option<u64>,
}

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

impl EbmlNumber {
    fn is_nan(&self) -> bool {
        self.raw_value == (0b1 << (self.length * 7)) - 1
    }
    fn as_id(&self) -> EbmlElementRawId {
        self.raw_value | (0b1 << (self.length * 7))
    }
    fn as_unsigned(&self) -> u64 {
        self.raw_value
    }
}


pub mod database;
pub mod parse;
