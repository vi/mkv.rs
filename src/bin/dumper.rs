
#![allow(non_camel_case_types)]
#![allow(unused_imports)]
use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use std::io::Read;
use std::io::Write;

use mkv::elements::parser::Parser;

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

    let path = Path::new("q.mkv");
    let mut f = match File::open(&path) {
        Ok(x) => BufReader::new(x),
        Err(_) => panic!("Failed to open file q.mkv"),
    };
    
    //let mut stdout = std::io::stdout();
    let element_logger = mkv::elements::parser::debug::DebugPrint::new(log::LogLevel::Info);
    let mut m = mkv::elements::parser::new(element_logger);
    
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
