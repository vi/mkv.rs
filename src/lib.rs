#![allow(unused_imports)]
#![allow(dead_code)]

#[macro_use]
extern crate nom;

use nom::{IResult, ErrorKind, be_u8, be_u16, be_u32};

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct EbmlNumber {
    pub raw_value: u64,
    pub length: u8,
}

type EbmlElementRawId = u64;

pub struct EbmlElementHeader {
    pub id: EbmlElementRawId,
    pub len: Option<u64>,
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

named!(pub ebml_header <EbmlElementHeader>,
    do_parse!(
        id: ebml_number >>
        len: ebml_number >>
        (EbmlElementHeader{
            id: id.as_id(),
            len: if len.is_nan() { None} else { Some(len.as_unsigned() )}
        })
    )
);


named!(pub ebml_number <EbmlNumber>,
    alt!(
        do_parse!(
            x: bits!(preceded!(tag_bits!(u8,   1, 0b1),
                              take_bits!(u8, 8-1))) >>
            (EbmlNumber{raw_value:x as u64, length: 1})
        ) |
        do_parse!(
            x: bits!(preceded!(tag_bits!(u8,   2, 0b01),
                              take_bits!(u8, 8-2))) >>
            y: be_u8 >>
            (EbmlNumber{raw_value:((x as u64) << 8) | (y as u64), length:  2})
        ) |
        do_parse!(
            x: bits!(preceded!(tag_bits!(u8,   3, 0b001),
                              take_bits!(u8, 8-3))) >>
            y: be_u16 >>
            (EbmlNumber{raw_value:((x as u64) << 16) | (y as u64), length:  3})
        ) |
        do_parse!(
            x: bits!(preceded!(tag_bits!(u8,   4, 0b0001),
                              take_bits!(u8, 8-4))) >>
            y: be_u8 >>
            z: be_u16 >>
            (EbmlNumber{raw_value:((x as u64) << 24) | ((y as u64) << 16) | (z as u64), length:  4})
        ) |
        do_parse!(
            x: bits!(preceded!(tag_bits!(u8,   5, 0b00001),
                              take_bits!(u8, 8-5))) >>
            y: be_u16 >>
            z: be_u16 >>
            (EbmlNumber{raw_value:((x as u64) << 32) | ((y as u64) << 16) | (z as u64), length: 5})
        ) |
        do_parse!(
            x: bits!(preceded!(tag_bits!(u8,   6, 0b000001),
                              take_bits!(u8, 8-6))) >>
            y: be_u8 >>
            z: be_u32 >>
            (EbmlNumber{raw_value:((x as u64) << 40) | ((y as u64) << 32) | (z as u64), length:  6})
        ) |
        do_parse!(
            x: bits!(preceded!(tag_bits!(u8,   7, 0b0000001),
                              take_bits!(u8, 8-7))) >>
            y: be_u16 >>
            z: be_u32 >>
            (EbmlNumber{raw_value:((x as u64) << 48) | ((y as u64) << 32) | (z as u64), length:  7})
        ) |
        do_parse!(
            tag!(b"\x01") >>
            y: be_u8 >>
            z: be_u16 >>
            a: be_u32 >>
            (EbmlNumber{raw_value:((y as u64) << 48) | ((z as u64)<<32) | (a as u64), length:  8})
        )
    )
);


mod test;
