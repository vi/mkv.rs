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
}
