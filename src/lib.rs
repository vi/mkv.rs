#![allow(non_camel_case_types)]
#![allow(unused_imports)]
#![allow(unused_parens)]
#![allow(unused_mut)]
#![allow(unused_variables)]

#![cfg_attr(feature = "rustc-serialize", feature(custom_derive))]

#[macro_use]
extern crate log;

#[cfg(feature = "rustc-serialize")] extern crate rustc_serialize;

pub mod elements;
