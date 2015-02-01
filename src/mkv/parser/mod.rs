use mkv::EventsHandler;
use mkv::Parser;
use mkv::AuxilaryEvent::Debug;

mod tests_parse_ebml_number;

#[derive(PartialEq,Show)] // for assert_eq!
enum EbmlParseNumberResult {
    Error,
    NotEnoughData,
    NaN,
    Ok(u64),
}

enum EbmlParseNumberMode {
    Unsigned,
    Identifier,
}

/** returns the rest of input buffer which is unused */
fn parse_ebml_number(bytes:&[u8], mode:EbmlParseNumberMode) -> (EbmlParseNumberResult, &[u8])
{
    use self::EbmlParseNumberResult::*;
    use self::EbmlParseNumberMode::*;
    
    let mut b =     bytes.iter();
    let firstbyte = match b.next() {
                Some(x) => *x, 
                None => return (NotEnoughData, bytes)
    };
    let mut more_bytes : usize;
    let mut mask : u8;
    
    if      firstbyte & 0x80 != 0 { more_bytes = 0; mask = 0x7F; }
    else if firstbyte & 0x40 != 0 { more_bytes = 1; mask = 0x3F; }
    else if firstbyte & 0x20 != 0 { more_bytes = 2; mask = 0x1F; }
    else if firstbyte & 0x10 != 0 { more_bytes = 3; mask = 0x0F; }
    else if firstbyte & 0x08 != 0 { more_bytes = 4; mask = 0x07; }
    else if firstbyte & 0x04 != 0 { more_bytes = 5; mask = 0x03; }
    else if firstbyte & 0x02 != 0 { more_bytes = 6; mask = 0x01; }
    else if firstbyte & 0x01 != 0 { more_bytes = 7; mask = 0x00; }
    else { return (Error, bytes); }
    
    let mut x = match mode {
        Unsigned   => (firstbyte & mask) as u64,
        Identifier =>  firstbyte         as u64,
    };
    let mut is_nan = (firstbyte & mask) == (0xFF & mask);
    
    for _ in (0..more_bytes) {
        x <<= 8;
        let nextbyte = match b.next() {
            Some(v) => *v,
            None => return (NotEnoughData, bytes)
        };
        x += nextbyte as u64;
        if nextbyte != 0xFF { is_nan = false; }
    };
    
    let (_, rest) = bytes.split_at(1+more_bytes);
    
    if is_nan {
        (NaN, rest)
    } else {
        (Ok(x), rest)
    }
}

pub struct OpenedElement {
    id : u64,
    offset : u64,
    length : Option<u64>,
}

pub struct ParserState<E> {
    cb : E,
    accumulator : Vec<u8>,
    opened_elements_stack : Vec<OpenedElement>,
    resyncing : bool,
    current_offset : u64,
}


enum ResultOfTryParseSomething<'a> {
    KeepGoing(&'a [u8]),
    NoMoreData,
    Error,
}

impl<E:EventsHandler> ParserState<E> {
    fn try_parse_something<'a>(&mut self, buf:&'a [u8]) -> ResultOfTryParseSomething<'a> {
        use self::ResultOfTryParseSomething::{NoMoreData,KeepGoing};
        use self::ResultOfTryParseSomething::Error as MyError;
        use self::EbmlParseNumberResult::*;
        use self::EbmlParseNumberMode::*;
        
        let (r1, restbuf) = parse_ebml_number(buf, Identifier);
        let element_id = match r1 {
            Error => return MyError,
            NaN => return MyError,
            NotEnoughData => return NoMoreData,
            Ok(x) => x
        };
        
        let (r2, restbuf2) = parse_ebml_number(restbuf, Unsigned);
        let element_size = match r2 {
            Error => return MyError,
            NaN => None,
            NotEnoughData => return NoMoreData,
            Ok(x) => Some(x)
        };
        
        NoMoreData
    }
}


impl<E:EventsHandler> Parser<E> for ParserState<E> {
    fn initialize(cb : E) -> ParserState<E> {
        ParserState {
            accumulator: vec![],
            cb : cb,
            resyncing : false,
            opened_elements_stack : vec![],
            current_offset : 0,
        }
    }
    
    fn feed_bytes(&mut self, bytes : &[u8])
    {
        use self::ResultOfTryParseSomething::*;
        
        self.accumulator.push_all(bytes);
        
        let mut buf = bytes.as_slice();
        loop {
            let r = self.try_parse_something(buf);
            match r {
                NoMoreData => return,
                Error => panic!("Need to implement resyncing"),
                KeepGoing(rest) => buf = rest,
            }
        }
        //self.cb.auxilary_event( Debug (format!("feed_bytes {} len={}", bytes[0], self.accumulator.len()) ));
    }
}
