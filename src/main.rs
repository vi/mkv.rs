use std::io::File;
use std::io::BufferedReader;

use mkvparse::MkvParser;

mod mkvparse;

struct Dummy;

impl mkvparse::MkvCallbacks for Dummy{
    fn debug(&self, str : String) {
        println!("debug: {}", str);
    }
}


fn main() {
    let path = Path::new("q.mkv");
    let mut f = BufferedReader::new(File::open(&path));
    
    let mut mkv : mkvparse::State<Dummy> = MkvParser::initialize(Dummy);
    
    match f.read_byte() {
        Ok(x) => mkv.feed_bytes(vec![x]),
        Err(e) => println!("error reading: {}", e)
    }
}
