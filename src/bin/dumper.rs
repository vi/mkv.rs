#![feature(core)]
#![feature(convert)]
#![feature(collections)]
#![feature(slice_patterns)]
#![feature(vec_push_all)]
#![feature(append)]

#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::io::Read;
use std::io::Write;

use mkv::elements::parser::Parser;

extern crate mkv;

const BSIZE : usize = 4096;
fn main() {
    let path = Path::new("q.mkv");
    let mut f = match File::open(&path) {
        Ok(x) => BufReader::new(x),
        Err(e) => panic!("Failed to open file q.mkv"),
    };
    
    let mut stdout = std::io::stdout();
    let mut m = mkv::elements::parser::new(mkv::elements::parser::debug::debug_logger(&mut stdout as &mut Write));
    
    loop {
        let mut b = [0; BSIZE];
        match f.read(&mut b) {
            Ok(x) => match x {
                    0 => break,
                    x => m.feed_bytes(b.split_at(x).0),
                },
            Err(e) => { println!("error reading: {}", e); break; },
        }
    }
}
