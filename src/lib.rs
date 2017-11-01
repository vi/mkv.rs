#![allow(unused_imports)]
#![allow(dead_code)]

#[macro_use]
extern crate nom;


pub mod database;
pub mod parse;

pub use database::ClassName;

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

#[derive(Debug, PartialEq, Clone)]
pub enum MatroskaParserEvent<'a> {
    HeaderOfALargeElementEncountered(ClassName),
    SmallElement(ClassName, ElementContent<'a>),
}

#[derive(PartialEq,Debug,PartialOrd,Clone)]
pub enum ElementContent<'a> {
    Master(Vec<(ClassName, ElementContent<'a>)>),
    Unsigned(u64),
    Signed(i64),
    Binary(&'a [u8]),
    Text(&'a str),
    Float(f64),
    MatroskaDate(i64), // Nanoseconds since 20010101_000000_UTC
    Unknown(u64, &'a [u8]),
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
    fn as_signed(&self) -> i64 {
        if 0 == self.raw_value & (0b1 << (self.length * 7 - 1)) {
            self.raw_value as i64
        } else {
            (self.raw_value as i64) - (0b1 << (self.length * 7))
        }
    }
}

