use super::*;

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
    assert_eq!(EbmlNumber {
                       raw_value: 0x7F,
                       length: 1,
                   }
                   .is_nan(),
               true);
    assert_eq!(EbmlNumber {
                       raw_value: 0x7F,
                       length: 2,
                   }
                   .is_nan(),
               false);
    assert_eq!(EbmlNumber {
                       raw_value: 0x00,
                       length: 1,
                   }
                   .is_nan(),
               false);
    assert_eq!(EbmlNumber {
                       raw_value: 0x01,
                       length: 1,
                   }
                   .is_nan(),
               false);
    assert_eq!(EbmlNumber {
                       raw_value: 0x7E,
                       length: 1,
                   }
                   .is_nan(),
               false);
    assert_eq!(EbmlNumber {
                       raw_value: 0x3FFF,
                       length: 2,
                   }
                   .is_nan(),
               true);
    assert_eq!(EbmlNumber {
                       raw_value: 0x3FFF,
                       length: 3,
                   }
                   .is_nan(),
               false);
    assert_eq!(EbmlNumber {
                       raw_value: 0x3FFF,
                       length: 4,
                   }
                   .is_nan(),
               false);
    assert_eq!(EbmlNumber {
                       raw_value: 0x3FFE,
                       length: 2,
                   }
                   .is_nan(),
               false);
    assert_eq!(EbmlNumber {
                       raw_value: 0x0000,
                       length: 2,
                   }
                   .is_nan(),
               false);
    assert_eq!(EbmlNumber {
                       raw_value: 0x1FFFFF,
                       length: 3,
                   }
                   .is_nan(),
               true);
    assert_eq!(EbmlNumber {
                       raw_value: 0x1FFFFF,
                       length: 4,
                   }
                   .is_nan(),
               false);
    assert_eq!(EbmlNumber {
                       raw_value: 0x0FFFFFFF,
                       length: 4,
                   }
                   .is_nan(),
               true);
    assert_eq!(EbmlNumber {
                       raw_value: 0x07FFFFFFFF,
                       length: 5,
                   }
                   .is_nan(),
               true);
    assert_eq!(EbmlNumber {
                       raw_value: 0x03FFFFFFFFFF,
                       length: 6,
                   }
                   .is_nan(),
               true);
    assert_eq!(EbmlNumber {
                       raw_value: 0x01FFFFFFFFFFFF,
                       length: 7,
                   }
                   .is_nan(),
               true);
    assert_eq!(EbmlNumber {
                       raw_value: 0x00FFFFFFFFFFFFFF,
                       length: 8,
                   }
                   .is_nan(),
               true);
    assert_eq!(EbmlNumber {
                       raw_value: 0x00FFFFFFFFFFFFFE,
                       length: 8,
                   }
                   .is_nan(),
               false);
}

#[test]
fn t_asid() {
    assert_eq!(EbmlNumber {
                       raw_value: 0x0A,
                       length: 1,
                   }
                   .as_id(),
               0x8A);
    assert_eq!(EbmlNumber {
                       raw_value: 0x0A45DFA3,
                       length: 4,
                   }
                   .as_id(),
               0x1A45DFA3);

}

#[test]
fn t_asid2() {
    assert_eq!(ebml_number(b"\x1A\x45\xDF\xA3").unwrap().1.as_id(),
               0x1A45DFA3);
    assert_eq!(ebml_number(b"\x54\xBB").unwrap().1.as_id(),
               0x54BB);
}

#[test]
fn t_signed() {
    assert_eq!(ebml_number(b"\x82")
                .unwrap().1.as_signed(), 2);
    assert_eq!(ebml_number(b"\xFF")
                .unwrap().1.as_signed(), -1);
    assert_eq!(ebml_number(b"\xBF")
                .unwrap().1.as_signed(), 63);
    assert_eq!(ebml_number(b"\xC0")
                .unwrap().1.as_signed(), -64);
    assert_eq!(ebml_number(b"\x7F\xFF")
                .unwrap().1.as_signed(), -1);
    assert_eq!(ebml_number(b"\x5F\xFF")
                .unwrap().1.as_signed(), 8191);
    assert_eq!(ebml_number(b"\x60\x00")
                .unwrap().1.as_signed(), -8192);
    assert_eq!(ebml_number(b"\x3F\xFF\xFF")
                .unwrap().1.as_signed(), -1);
    assert_eq!(ebml_number(b"\x1F\xFF\xFF\xFF")
                .unwrap().1.as_signed(), -1);
    assert_eq!(ebml_number(b"\x0F\xFF\xFF\xFF\xFF")
                .unwrap().1.as_signed(), -1);
    assert_eq!(ebml_number(b"\x07\xFF\xFF\xFF\xFF\xFF")
                .unwrap().1.as_signed(), -1);
    assert_eq!(ebml_number(b"\x03\xFF\xFF\xFF\xFF\xFF\xFF")
                .unwrap().1.as_signed(), -1);
    assert_eq!(ebml_number(b"\x01\xFF\xFF\xFF\xFF\xFF\xFF\xFF")
                .unwrap().1.as_signed(), -1);
}

#[test]
fn t_head1() {
    assert_eq!(ebml_header(b"\x84\x83\x01\x02\x03"),
               IResult::Done(&b"\x01\x02\x03"[..],
                             EbmlElementHeader {
                                 id: 0x84,
                                 len: Some(3),
                             }));
    assert_eq!(ebml_header(b"\x84\xFF\x01\x02\x03"),
               IResult::Done(&b"\x01\x02\x03"[..],
                             EbmlElementHeader {
                                 id: 0x84,
                                 len: None,
                             }));
}
