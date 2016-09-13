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
extern crate env_logger;

const BSIZE : usize = 65536;
fn main() {  
    env_logger::init().unwrap();

    let reader : Box<Read> = match args().len() {
        1 => Box::new(std::io::stdin()),
        2 => Box::new(File::open(Path::new(args().nth(1).unwrap().as_str())).expect("Failed to open the file")),
        _ => panic!("Usage: remux1 [input.mkv] > output.mkv")
    };
    let mut f = BufReader::new(reader);
   
    
    let stdout = std::io::stdout();
    let events_to_file = mkv::elements::midlevel::MidlevelEventsToFile::new(stdout);
    let mut midlevel = mkv::elements::midlevel::MidlevelParser::new(events_to_file);
    
    {
        let mut m = mkv::elements::parser::new();
        
        loop {
            let mut b = [0; BSIZE];
            match f.read(&mut b) {
                Ok(x) => match x {
                        0 => break,
                        x => m.feed_bytes(b.split_at(x).0, &mut midlevel),
                    },
                Err(e) => { println!("error reading: {}", e); break; },
            }
        }
    }
}
