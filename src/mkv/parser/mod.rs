use mkv::EventsHandler;
use mkv::Parser;
use mkv::AuxilaryEvent::Debug;

mod tests_parse_ebml_number;

// I expect that filled in, but not accessed fields would get optimized away
struct EbmlNumberInfo {
    as_id       : u64,
    as_unsigned : u64,
    length_in_bytes : usize,
}

enum EbmlParseNumberResult {
    Error,
    NotEnoughData,
    NaN(usize), // length_in_bytes
    Ok(EbmlNumberInfo)
}

fn parse_ebml_number(bytes:&[u8]) -> EbmlParseNumberResult {
    use self::EbmlParseNumberResult::*;
    
    let mut b =     bytes.iter();
    let firstbyte = match b.next() {
                Some(x) => *x, 
                None => return NotEnoughData
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
    else { return Error }
    
    let mut as_id       = firstbyte as u64;
    let mut as_unsigned = (firstbyte & mask) as u64;
    let mut is_nan = (firstbyte & mask) == (0xFF & mask);
    
    for _ in (0..more_bytes) {
        as_unsigned <<= 8;
        as_id       <<= 8;
        let nextbyte = match b.next() {
            Some(x) => *x,
            None => return NotEnoughData
        };
        as_unsigned += nextbyte as u64;
        as_id       += nextbyte as u64;
        if nextbyte != 0xFF { is_nan = false; }
    };
    
    if is_nan {
        NaN(1+more_bytes)
    } else {
        Ok(EbmlNumberInfo {
            as_id       : as_id,
            as_unsigned : as_unsigned,
            length_in_bytes : 1+more_bytes,
        })
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


enum ResultOfTryParseSomething {
    KeepGoing,
    NoMoreData,
    Error,
}

impl<E:EventsHandler> ParserState<E> {
    fn try_parse_something(&mut self) -> ResultOfTryParseSomething {
        use self::ResultOfTryParseSomething::{NoMoreData,KeepGoing};
        use self::ResultOfTryParseSomething::Error as MyError;
        use self::EbmlParseNumberResult::*;
        
        let buf = self.accumulator.as_slice();
        let (elemend_id, offs) = match parse_ebml_number(buf) {
            Error => return MyError,
            NaN(_) => return MyError,
            NotEnoughData => return NoMoreData,
            Ok(x) => (x.as_id, x.length_in_bytes),
        };
        let (_, restbuf) = buf.split_at(offs);
        let (element_size, offs) = match parse_ebml_number(restbuf) {
            Error => return MyError,
            NaN(n) => (None, n),
            NotEnoughData => return NoMoreData,
            Ok(x) => (Some(x.as_unsigned), x.length_in_bytes),
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
        loop {
            let r = self.try_parse_something();
            match r {
                NoMoreData => return,
                Error => panic!("Need to implement resyncing"),
                KeepGoing => (),
            }
        }
        //self.cb.auxilary_event( Debug (format!("feed_bytes {} len={}", bytes[0], self.accumulator.len()) ));
    }
}
