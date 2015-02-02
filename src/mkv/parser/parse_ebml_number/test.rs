use super::parse_ebml_number;
use super::Result;
use super::Mode::*;


#[test]
fn parse_ebml_number_test_1() {
    let input = [0x1A,0x45,0xDF,0xA3];
    let (r,rest) = parse_ebml_number(&input, Identifier);
    assert_eq!(r, Result::Ok(0x1A45DFA3));
    assert_eq!(rest, []);
}

#[test]
fn parse_ebml_number_test_2() {
    let input = [0x1A,0x45,0xDF,0xA3];
    let (r,rest) = parse_ebml_number(&input, Unsigned);
    assert_eq!(r, Result::Ok(0x0A45DFA3));
    assert_eq!(rest, []);
}


#[test]
fn parse_ebml_number_test_3() {
    let input = [0x4A,0x45,0xDF,0xA3];
    let (r,rest) = parse_ebml_number(&input, Unsigned);
    assert_eq!(r, Result::Ok(0x0A45));
    assert_eq!(rest, [0xDF,0xA3]);
}


#[test]
fn parse_ebml_number_test_4() {
    let input = [0xFF,0x7F];
    let (r,rest) = parse_ebml_number(&input, Unsigned);
    assert_eq!(r, Result::NaN);
    assert_eq!(rest, [0x7F]);
}


#[test]
fn parse_ebml_number_test_5() {
    let input = [0x7F,0xFF];
    let (r,rest) = parse_ebml_number(&input, Unsigned);
    assert_eq!(r, Result::NaN);
    assert_eq!(rest, []);
}

#[test]
fn parse_ebml_number_test_6() {
    let input = [0x00];
    let (r,rest) = parse_ebml_number(&input, Unsigned);
    assert_eq!(r, Result::Error);
}

#[test]
fn parse_ebml_number_test_7() {
    let input = [0x40];
    let (r,rest) = parse_ebml_number(&input, Unsigned);
    assert_eq!(r, Result::NotEnoughData);
}

#[test]
fn parse_ebml_number_test_8() {
    let input = [];
    let (r,rest) = parse_ebml_number(&input, Identifier);
    assert_eq!(r, Result::NotEnoughData);
}
