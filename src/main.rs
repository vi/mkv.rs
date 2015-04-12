#![feature(core)]
#![feature(convert)]
#![feature(collections)]
#![feature(slice_patterns)]

#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::io::Read;

use mkv::elements::parser::Parser;

mod mkv;

struct MyHandlerState {
    ctr : i32,
    indent : usize,
}

impl mkv::elements::parser::EventsHandler for MyHandlerState {
    fn event(&mut self, e : mkv::elements::parser::Event) {
        use mkv::elements::parser::Event::*;
        
        match e {
            End(_) => if self.indent > 0 { self.indent -= 1 },
            _ => (),
        }
        for _ in 0..self.indent { print!(" "); }
        match e {
            Begin(x) => println!("element {:?}", x),
            Data(ref x) => println!("data {:?}", x),
            End(x) => println!("end {:?}", x),
            Resync => println!("resync"),
        }
        match e {
            Begin(_) => self.indent += 1,
            _ => (),
        }
        
        self.ctr+=1;
    }
    fn log(&mut self, t : &str) {
        println!("log: {}", t);
    }
}

const BSIZE : usize = 4096;
fn main() {
    let path = Path::new("q.mkv");
    let mut f = match File::open(&path) {
        Ok(x) => BufReader::new(x),
        Err(e) => panic!("Failed to open file q.mkv"),
    };
    
    let du = MyHandlerState { ctr: 0, indent : 0 };
    let mut m = mkv::elements::parser::new(du);
    
    
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
