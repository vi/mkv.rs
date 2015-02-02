mod test;

#[derive(PartialEq,Show)] // for assert_eq!
pub enum Result {
    Error,
    NotEnoughData,
    NaN,
    Ok(u64),
}

pub enum Mode {
    Unsigned,
    Identifier,
}

/** returns the rest of input buffer which is unused */
pub fn parse_ebml_number(bytes:&[u8], mode:Mode) -> (Result, &[u8])
{
    use self::Result::*;
    use self::Mode::*;
    
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
