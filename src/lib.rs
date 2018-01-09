#![allow(non_camel_case_types)]
#![allow(unused_imports)]
#![allow(unused_parens)]
#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(unused_macros)]

#![cfg_attr(feature = "rustc-serialize", feature(custom_derive))]
#![cfg_attr(feature = "nightly", feature(fn_traits))]

#[macro_use]
extern crate log;

extern crate bytes;

#[cfg(feature = "rustc-serialize")] extern crate rustc_serialize;

pub mod elements;
pub mod events;
