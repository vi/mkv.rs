use std::io::File;
use std::io::BufferedReader;

use mkvparse::MkvParser;

mod mkvparse;

struct Dummy {
    ctr : i32,
}

impl mkvparse::MkvCallbacks for Dummy{
    fn debug(&mut self, str : String) {
        println!("debug {}: {}", self.ctr, str);
        self.ctr+=1;
    }
}


fn main() {
    let path = Path::new("q.mkv");
    let mut f = BufferedReader::new(File::open(&path));
    
    let du = Dummy { ctr: 0 };
    let mut mkv : mkvparse::State<Dummy> = MkvParser::initialize(du);
    
    loop {
        match f.read_byte() {
            Ok(x) => mkv.feed_bytes(vec![x]),
            Err(e) => { println!("error reading: {}", e); break; }
        }
    }
}
