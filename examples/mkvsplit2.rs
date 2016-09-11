#![allow(non_camel_case_types)]
#![allow(unused_imports)]

use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::io::Read;
use std::io::Write;
use std::env::args;

use mkv::elements::parser::Parser;
use mkv::elements::parser::EventsHandler;
use mkv::elements::Element;
use mkv::elements::database::Class;
use mkv::elements::database::class_to_id;
use mkv::elements::generator;
use mkv::elements::el_bin;
use mkv::elements::generator::generate;
use mkv::elements::generator::EbmlNumberMode;
use mkv::elements::generator::generate_ebml_number;

extern crate mkv;
extern crate env_logger;

use std::rc::Rc;

struct MyHandler {
    counter : usize,
    w : Option<Box<Write>>,
}

use mkv::elements::midlevel::MidlevelEventHandler;
use mkv::elements::midlevel::MidlevelEvent;
use mkv::elements::midlevel::WhatToDo;
use mkv::elements::typical_matroska_header;

impl MidlevelEventHandler for MyHandler {

    fn handle(&mut self, e: MidlevelEvent) -> WhatToDo {
        match e {
            MidlevelEvent::EnterElement(klass) => {
                if klass == Class::Segment {
                    let mut w;
                    {
                        let filen = format!("{}.mkv",self.counter);
                        w = File::create(Path::new(&filen)).expect("Can't open output file");
                    }
                    
                    w.write(&generate(&typical_matroska_header(false))).unwrap();
                    w.write(&generate_ebml_number(class_to_id(klass), EbmlNumberMode::Identifier)).unwrap();
                    w.write(b"\xFF").unwrap(); // unknown length
                    w.write(&generate(&el_bin(Class::Void, vec![0;32]))).unwrap();
                    
                    self.w = Some(Box::new(w));
                    self.counter += 1;
                    WhatToDo::GoOn
                } else {
                    WhatToDo::Build
                }
            }
            MidlevelEvent::Element(x) => {
                if let Some(ref mut w) = self.w {
                    w.write(&generate(&x)).unwrap();
                }
                WhatToDo::GoOn
            }
            MidlevelEvent::LeaveElement(_) => { 
                WhatToDo::GoOn
            }
            MidlevelEvent::Resync => {
                if let Some(ref mut w) = self.w {
                    w.write(&generate(&el_bin(Class::Void, b"\nHere was resync\n".to_vec()))).unwrap();
                }
                WhatToDo::GoOn
            }
        }
    }
}

const BSIZE : usize = 4096;
fn main() {
    env_logger::init().unwrap();

    let reader : Box<Read> = match args().len() {
        1 => Box::new(std::io::stdin()),
        2 => Box::new(File::open(Path::new(args().nth(1).unwrap().as_str())).expect("Failed to open the file")),
        _ => panic!("Usage: mkvsplit2 [filename.mkv]\nExtracts segments to 1.mkv, 2.mkv and so on\n")
    };
    let mut f = BufReader::new(reader);
   
    
    //let mut stdout = std::io::stdout();
    let myhandler = MyHandler { counter: 1, w: None, };
    let mut midlevel = mkv::elements::midlevel::MidlevelParser::new(myhandler);
    
    {
        let mut m = mkv::elements::parser::new(&mut midlevel);
        
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
}
