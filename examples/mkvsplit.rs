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
use mkv::elements::generator;

extern crate mkv;
extern crate log;

use std::rc::Rc;

struct SimpleLogger;
impl log::Log for SimpleLogger {
    fn enabled(&self, _: &log::LogMetadata) -> bool { true     }
    fn log(&self, record: &log::LogRecord) { println!("{}", record.args());  }
}

const BSIZE : usize = 4096;
fn main() {
    let _ = log::set_logger(|ll| { ll.set(log::LogLevelFilter::Debug); Box::new(SimpleLogger) });

    let reader : Box<Read> = match args().len() {
        1 => Box::new(std::io::stdin()),
        2 => Box::new(File::open(Path::new(args().nth(1).unwrap().as_str())).expect("Failed to open the file")),
        _ => panic!("Usage: mkvsplit [filename.mkv]\nExtracts segments to 1.mkv, 2.mkv and so on\n")
    };
    let mut f = BufReader::new(reader);
   
    
    //let mut stdout = std::io::stdout();
    let mut dom_builder : mkv::elements::builder::Builder = Default::default();
    {
        let mut m = mkv::elements::parser::new(&mut dom_builder);
        
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
    
    let ce : Vec<Rc<Element>> = dom_builder.into_captured_elements();
    
    let ebml_head = &ce[0];
    assert_eq!(ebml_head.class, Class::EBML);
    
    let mut counter = 1;
    
    for i in &ce[1..] {
        assert_eq!(i.class, Class::Segment);
        let mut w = File::create(Path::new(&format!("{}.mkv",counter))).unwrap();
        w.write(&generator::generate(ebml_head)).unwrap();
        w.write(&generator::generate(i)).unwrap();
        counter+=1;
    }
    
}
