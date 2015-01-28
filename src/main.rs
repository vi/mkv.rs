use std::io::File;
use std::io::BufferedReader;

use mkv::Parser;

mod mkv;

struct MyHandlerState {
    ctr : i32,
}

impl mkv::EventsHandler for MyHandlerState {
    fn auxilary_event(&mut self, e : mkv::AuxilaryEvent) {
        match e {
            mkv::AuxilaryEvent::Warning(str) => println!("warning {}", str),
            mkv::AuxilaryEvent::Debug(str)   => println!("debug {}: {}", self.ctr, str),
        }
        self.ctr+=1;
    }
}


fn main() {
    let path = Path::new("q.mkv");
    let mut f = BufferedReader::new(File::open(&path));
    
    let du = MyHandlerState { ctr: 0 };
    let mut m : mkv::parser::ParserState<MyHandlerState> = mkv::Parser::initialize(du);
    
    loop {
        match f.read_byte() {
            Ok(x) => m.feed_bytes(vec![x]),
            Err(e) => { println!("error reading: {}", e); break; }
        }
    }
}
