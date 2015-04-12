#![feature(collections,convert,core)]
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::io::Read;

use mkv::ElementParser;

mod mkv;

struct MyHandlerState {
    ctr : i32,
    indent : usize,
}

impl<'a> mkv::ElementEventsHandler<'a> for MyHandlerState {
    fn event(&mut self, e : mkv::ElementEvent) {
        use mkv::ElementEvent::*;
        
        match e {
            ElementEnd(_) => if self.indent > 0 { self.indent -= 1 },
            _ => (),
        }
        for _ in 0..self.indent { print!(" "); }
        match e {
            ElementBegin(x) => println!("element {:?}", x),
            ElementData(ref x) => println!("data {:?}", x),
            ElementEnd(x) => println!("end {:?}", x),
            Resync => println!("resync"),
        }
        match e {
            ElementBegin(_) => self.indent += 1,
            _ => (),
        }
        
        self.ctr+=1;
    }
    fn log(&mut self, t : &str) {
        println!("log: {}", t);
    }
}

fn main() {
    let path = Path::new("q.mkv");
    let mut f = match File::open(&path) {
        Ok(x) => BufReader::new(x),
        Err(e) => panic!("Failed to open file q.mkv"),
    };
    
    let du = MyHandlerState { ctr: 0, indent : 0 };
    let mut m : mkv::parser::ParserState<MyHandlerState> = mkv::ElementParser::initialize(du);
    
    
    loop {
        let mut b = [0; 4096];
        match f.read(&mut b) {
            Ok(x) => m.feed_bytes(&b),
            Err(e) => { println!("error reading: {}", e); break; },
        }
    }
}
