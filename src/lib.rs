#![allow(unused_imports)]
#![allow(dead_code)]

#[macro_use]
extern crate nom;

use nom::{IResult, ErrorKind, be_u8, be_u16, be_u32};

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct EbmlNumber {
    raw_value: u64,
    length: u8,
}

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

impl EbmlNumber {
    fn is_nan(&self) -> bool {
        self.raw_value == (0b1 << (self.length*7)) - 1
    }
}

#[test]
fn eun() {
    assert_eq!(ebml_number(b"\x8A\x45\xDF\xA3"),
               IResult::Done(&b"\x45\xDF\xA3"[..],
                             EbmlNumber {
                                 raw_value: 0x0A,
                                 length: 1,
                             }));
    assert_eq!(ebml_number(b"\x4A\x45\xDF\xA3"),
               IResult::Done(&b"\xDF\xA3"[..],
                             EbmlNumber {
                                 raw_value: 0x0A45,
                                 length: 2,
                             }));
    assert_eq!(ebml_number(b"\x2A\x45\xDF\xA3"),
               IResult::Done(&b"\xA3"[..],
                             EbmlNumber {
                                 raw_value: 0x0A45DF,
                                 length: 3,
                             }));
    assert_eq!(ebml_number(b"\x1A\x45\xDF\xA3"),
               IResult::Done(&b""[..],
                             EbmlNumber {
                                 raw_value: 0x0A45DFA3,
                                 length: 4,
                             }));
    assert_eq!(ebml_number(b"\x09\x45\xDF\xA3\x01\x02\x03\x04"),
               IResult::Done(&b"\x02\x03\x04"[..],
                             EbmlNumber {
                                 raw_value: 0x0145DFA301,
                                 length: 5,
                             }));
    assert_eq!(ebml_number(b"\x05\x45\xDF\xA3\x01\x02\x03\x04"),
               IResult::Done(&b"\x03\x04"[..],
                             EbmlNumber {
                                 raw_value: 0x0145DFA30102,
                                 length: 6,
                             }));
    assert_eq!(ebml_number(b"\x03\x45\xDF\xA3\x01\x02\x03\x04"),
               IResult::Done(&b"\x04"[..],
                             EbmlNumber {
                                 raw_value: 0x0145DFA3010203,
                                 length: 7,
                             }));
    assert_eq!(ebml_number(b"\x01\x45\xDF\xA3\x01\x02\x03\x04"),
               IResult::Done(&b""[..],
                             EbmlNumber {
                                 raw_value: 0x45DFA301020304,
                                 length: 8,
                             }));
}

#[test]
fn t_isnan() {
    assert_eq!(EbmlNumber{raw_value: 0x7F, length: 1}.is_nan(), true);
    assert_eq!(EbmlNumber{raw_value: 0x7F, length: 2}.is_nan(), false);
    assert_eq!(EbmlNumber{raw_value: 0x00, length: 1}.is_nan(), false);
    assert_eq!(EbmlNumber{raw_value: 0x01, length: 1}.is_nan(), false);
    assert_eq!(EbmlNumber{raw_value: 0x7E, length: 1}.is_nan(), false);
    assert_eq!(EbmlNumber{raw_value: 0x3FFF, length: 2}.is_nan(), true);
    assert_eq!(EbmlNumber{raw_value: 0x3FFF, length: 3}.is_nan(), false);
    assert_eq!(EbmlNumber{raw_value: 0x3FFF, length: 4}.is_nan(), false);
    assert_eq!(EbmlNumber{raw_value: 0x3FFE, length: 2}.is_nan(), false);
    assert_eq!(EbmlNumber{raw_value: 0x0000, length: 2}.is_nan(), false);
    assert_eq!(EbmlNumber{raw_value: 0x1FFFFF, length: 3}.is_nan(), true);
    assert_eq!(EbmlNumber{raw_value: 0x1FFFFF, length: 4}.is_nan(), false);
    assert_eq!(EbmlNumber{raw_value: 0x0FFFFFFF, length: 4}.is_nan(), true);
    assert_eq!(EbmlNumber{raw_value: 0x07FFFFFFFF, length: 5}.is_nan(), true);
    assert_eq!(EbmlNumber{raw_value: 0x03FFFFFFFFFF, length: 6}.is_nan(), true);
    assert_eq!(EbmlNumber{raw_value: 0x01FFFFFFFFFFFF, length: 7}.is_nan(), true);
    assert_eq!(EbmlNumber{raw_value: 0x00FFFFFFFFFFFFFF, length: 8}.is_nan(), true);
    assert_eq!(EbmlNumber{raw_value: 0x00FFFFFFFFFFFFFE, length: 8}.is_nan(), false);
}
