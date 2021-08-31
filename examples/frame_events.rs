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
use mkv::events::{MatroskaEventHandler, MatroskaFrame};

extern crate mkv;
extern crate env_logger;

struct My;


/*
pub struct MatroskaFrame {
    timecode_nanoseconds: u64,
    track_number: usize,
    /// outer vec - lacing, inner vec - byte buffer
    buffers: Vec<Vec<u8>>,
}
*/

impl MatroskaEventHandler for My {
    fn frame_encountered(&mut self, f: MatroskaFrame) {
        print!("frame ts={} tn={} len={}",
            (f.timecode_nanoseconds as f64)/1E9,
            f.track_number,
            f.buffers[0].len(),
        );
        if f.buffers[0].len()>=4 {
            print!("\t{:02x}{:02x}{:02x}{:02x}...",
                f.buffers[0][0],
                f.buffers[0][1],
                f.buffers[0][2],
                f.buffers[0][3],);
        }
        println!("");
    }

    fn segment_tracks(&mut self, _e: &std::rc::Rc<mkv::elements::Element>) {
        println!("(tracks description skipped)");
    }
}

const BSIZE : usize = 65536;
fn main() {  
    env_logger::init().unwrap();

    let reader : Box<dyn Read> = match args().len() {
        1 => Box::new(std::io::stdin()),
        2 => Box::new(File::open(Path::new(args().nth(1).unwrap().as_str())).expect("Failed to open the file")),
        _ => panic!("Usage: remux1 [input.mkv] > output.mkv")
    };
    let mut f = BufReader::new(reader);
   
    
    let m = My;
    let demuxer = mkv::events::MatroskaDemuxer::new(m);
    let mut midlevel = mkv::elements::midlevel::MidlevelParser::new(demuxer);
    
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
