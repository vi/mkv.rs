use mkv::EventsHandler;
use mkv::Parser;
use mkv::AuxilaryEvent::Debug;

mod tests;

// I expect that filled in, but not accessed fields would get optimized away
struct EbmlNumberInfo {
    as_id       : u64,
    as_unsigned : u64,
    length_in_bytes : isize,
}

enum EbmlParseNumberResult {
    Error,
    NaN(isize), // length_in_bytes
    Ok(EbmlNumberInfo)
}

fn parse_ebml_number(bytes: Vec<u8>) -> EbmlParseNumberResult {
    let mut b =     bytes.iter();
    let firstbyte = match b.next() {
                Some(x) => *x, 
                None => return EbmlParseNumberResult::Error
    };
    let mut more_bytes : isize;
    let mut mask : u8;
    
    if      firstbyte & 0x80 != 0 { more_bytes = 0; mask = 0x7F; }
    else if firstbyte & 0x40 != 0 { more_bytes = 1; mask = 0x3F; }
    else if firstbyte & 0x20 != 0 { more_bytes = 2; mask = 0x1F; }
    else if firstbyte & 0x10 != 0 { more_bytes = 3; mask = 0x0F; }
    else if firstbyte & 0x08 != 0 { more_bytes = 4; mask = 0x07; }
    else if firstbyte & 0x04 != 0 { more_bytes = 5; mask = 0x03; }
    else if firstbyte & 0x02 != 0 { more_bytes = 6; mask = 0x01; }
    else if firstbyte & 0x01 != 0 { more_bytes = 7; mask = 0x00; }
    else { return EbmlParseNumberResult::Error }
    
    let mut as_id       = firstbyte as u64;
    let mut as_unsigned = (firstbyte & mask) as u64;
    let mut is_nan = (firstbyte & mask) == (0xFF & mask);
    
    for i in (0..more_bytes) {
        as_unsigned <<= 8;
        as_id       <<= 8;
        let nextbyte = match b.next() {
            Some(x) => *x,
            None => return EbmlParseNumberResult::Error
        };
        as_unsigned += nextbyte as u64;
        as_id       += nextbyte as u64;
        if nextbyte != 0xFF { is_nan = false; }
    };
    
    if is_nan {
        EbmlParseNumberResult::NaN(1+more_bytes)
    } else {
        EbmlParseNumberResult::Ok(EbmlNumberInfo {
            as_id       : as_id,
            as_unsigned : as_unsigned,
            length_in_bytes : 1+more_bytes,
        })
    }
}


pub struct ParserState<E> {
    cb : E,
    accumulator : Vec<u8>,
}

impl<E:EventsHandler> Parser<E> for ParserState<E> {
    fn initialize(cb : E) -> ParserState<E> {
        ParserState {
            accumulator: vec![],
            cb : cb,
        }
    }
    
    fn feed_bytes(&mut self, bytes : Vec<u8>)
    {
        self.accumulator.push_all(bytes.as_slice());
        self.cb.auxilary_event( Debug (format!("feed_bytes {} len={}", bytes[0], self.accumulator.len()) ));
    }
}
