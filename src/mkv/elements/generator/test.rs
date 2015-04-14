use super::*;
use super::super::*;
use super::super::ElementContent::*;
use super::super::database::Class;
use std::rc::Rc;
use std::vec::Vec;

macro_rules! el_bin {  ($c:expr, $d:expr) => { Element { class: $c, content: Binary(Rc::new( $d )) }}  }

#[test] fn t1() { assert_eq!(generate(&el_bin!(Class::Void, vec![])), vec!(0xEC, 0x80)); }
#[test] fn t2() { assert_eq!(generate(&el_bin!(Class::Void, vec![0xFF])), vec!(0xEC, 0x81, 0xFF)); }
#[test] fn t3() { assert_eq!(generate(&el_bin!(Class::Void, vec![0x00,0x01])), vec!(0xEC, 0x82, 0x00,0x01)); }
#[test] fn t4() { assert_eq!(generate(
    &el_bin!(Class::SimpleBlock, vec![0x81,0x00] + &[0xFF ; 0x5000])),
    vec!(0xA3, 0x20, 0x50, 0x02, 0x81,0x00) + &[0xFF ; 0x5000]); }
