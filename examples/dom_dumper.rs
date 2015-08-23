#![feature(result_expect)]
#![feature(convert)]

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

extern crate mkv;
extern crate log;

struct SimpleLogger;
impl log::Log for SimpleLogger {
    fn enabled(&self, _: &log::LogMetadata) -> bool { true     }
    fn log(&self, record: &log::LogRecord) { println!("{}", record.args());  }
}

const BSIZE : usize = 4096;
fn main() {
    let _ = log::set_logger(|ll| { ll.set(log::LogLevelFilter::Debug); Box::new(SimpleLogger) });

    let mut reader : Box<Read> = match args().len() {
        1 => Box::new(std::io::stdin()),
        2 => Box::new(File::open(Path::new(args().nth(1).unwrap().as_str())).expect("Failed to open the file")),
        _ => panic!("Usage: event_dumper [filename.mkv]")
    };
    let mut f = BufReader::new(reader);
   
    
    //let mut stdout = std::io::stdout();
    let element_logger = mkv::elements::parser::debug::DebugPrint::new(log::LogLevel::Info);
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
    
    println!("{:#?}", dom_builder.captured_elements());
}
