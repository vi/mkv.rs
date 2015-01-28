use super::*;
use super::parse_ebml_number;
use super::EbmlParseNumberResult;
use super::EbmlNumberInfo;

fn unwrap_ebmlpr(x : EbmlParseNumberResult) -> EbmlNumberInfo {
    match(x) {
        EbmlParseNumberResult::Ok(y) => y,
        _ => panic!("Expected successfull parse")
    }
}

fn unwrap_nan(x : EbmlParseNumberResult) -> isize {
    match(x) {
        EbmlParseNumberResult::NaN(y) => y,
        _ => panic!("Expected NaN")
    }
}

#[test]
fn parse_ebml_number_test_1() {
    let rr = parse_ebml_number(vec![0x1A,0x45,0xDF,0xA3]);
    let r = unwrap_ebmlpr(rr);
    assert_eq!(r.as_id, 0x1A45DFA3);
    assert_eq!(r.as_unsigned, 0x0A45DFA3); 
    assert_eq!(r.length_in_bytes, 4); 
}


#[test]
fn parse_ebml_number_test_1b() {
    let rr = parse_ebml_number(vec![0x4A,0x45,0xDF,0xA3]);
    let r = unwrap_ebmlpr(rr);
    assert_eq!(r.as_id, 0x4A45);
    assert_eq!(r.as_unsigned, 0x0A45); 
    assert_eq!(r.length_in_bytes, 2); 
}

#[test]
fn parse_ebml_number_test_2() {
    let rr = parse_ebml_number(vec![0xFF,0x7F]);
    let r = unwrap_nan(rr);
    assert_eq!(r, 1); 
}
#[test]
fn parse_ebml_number_test_3() {
    let rr = parse_ebml_number(vec![0x7F,0xFF]);
    let r = unwrap_nan(rr);
    assert_eq!(r, 2); 
}

#[test]
fn parse_ebml_number_test_4() {
    let rr = parse_ebml_number(vec![0x00]);
    match rr {
        EbmlParseNumberResult::Error => (),
        _ => panic!("Expected error")
    }
}

#[test]
fn parse_ebml_number_test_5() {
    let rr = parse_ebml_number(vec![0x40]);
    match rr {
        EbmlParseNumberResult::Error => (),
        _ => panic!("Expected error")
    }
}
