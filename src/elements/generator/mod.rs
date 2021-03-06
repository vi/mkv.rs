use std::rc::Rc;
use std::vec::Vec;

use std::io::Write;

use super::*;
use super::database::Class;
use super::ElementContent::*;
extern crate byteorder;

mod test;

pub fn generate(x: &Element) -> Vec<u8>
{
    let mut r = vec!();
    let id = match x.content {
        Unknown(id_, _) => id_,
        _ => super::database::class_to_id(x.class),
    };
    let mut data : Vec<u8> = match x.content {
        Binary(ref x) => (**x).clone(),
        Unsigned(x) => generate_big_endian_number(x),
        Signed(x) => generate_big_endian_number_s(x),
        Float(x) => {
            use self::byteorder::WriteBytesExt;
            use self::byteorder::BigEndian;
            let mut vv = vec![];
            vv.write_f32::<BigEndian>(x as f32).unwrap();
            vv
        }
        Text(ref x) => (**x).clone().into_bytes(),
        Master(ref x) => {
            let mut vv = vec![];
            for i in x {
                vv.append( &mut generate(&*i) );
            }
            vv
        }
        MatroskaDate(x) => generate_big_endian_number_s(x),
        Unknown(_, ref x) => (**x).clone(),
    };
    
    r.append( &mut generate_ebml_number(id,                EbmlNumberMode::Identifier) );
    r.append( &mut generate_ebml_number(data.len() as u64, EbmlNumberMode::Unsigned) );
    r.append( &mut data );
    r
}

pub enum EbmlNumberMode {
    Unsigned,
    Identifier,
}

pub fn generate_ebml_number(x : u64, inputmode : EbmlNumberMode) -> Vec<u8>
{
    match inputmode {
        EbmlNumberMode::Identifier => generate_big_endian_number(x),
        EbmlNumberMode::Unsigned => {
            let mut r = vec!();
            if x == 0xFFFFFFFFFFFFFFFF { return vec!(0xFF); }
            
            let (numbytes, bit) = match x {
                v if v < 0x100             - 0x81             => (1, 0x80),
                v if v < 0x10000           - 0xC001           => (2, 0x40),
                v if v < 0x1000000         - 0xE00001         => (3, 0x20),
                v if v < 0x100000000       - 0xF0000001       => (4, 0x10),
                v if v < 0x10000000000     - 0xF800000001     => (5, 0x08),
                v if v < 0x1000000000000   - 0xFC0000000001   => (6, 0x04),
                v if v < 0x100000000000000 - 0xFE000000000001 => (7, 0x02),
                _                                             => (8, 0x01),
            };
            
            r.push( bit | ((x >> (numbytes-1)*8) as u8) );
            
            for i in 1..numbytes {
                r.push ( ((x >> ((numbytes-i-1)*8)) & 0xFF) as u8 );
            }
            r
        },
    }
}

pub fn generate_big_endian_number( x : u64) -> Vec<u8>
{
    let mut r = vec!();
    let numbytes = match x {
        v if v < 0x100              => 1,
        v if v < 0x10000            => 2,
        v if v < 0x1000000          => 3,
        v if v < 0x100000000        => 4,
        v if v < 0x10000000000      => 5,
        v if v < 0x1000000000000    => 6,
        v if v < 0x100000000000000  => 7,
        _                           => 8,
    };
    for i in 0..numbytes {
        r.push ( ((x >> ((numbytes-i-1)*8)) & 0xFF) as u8 );
    }
    r
}

pub fn generate_big_endian_number_s( x : i64) -> Vec<u8>
{
    let mut r = vec!();
    
    let numbytes = match x {
        v if -0x80             <= v && v < 0x80              => 1,
        v if -0x8000           <= v && v < 0x8000            => 2,
        v if -0x800000         <= v && v < 0x800000          => 3,
        v if -0x80000000       <= v && v < 0x80000000        => 4,
        v if -0x8000000000     <= v && v < 0x8000000000      => 5,
        v if -0x800000000000   <= v && v < 0x800000000000    => 6,
        v if -0x80000000000000 <= v && v < 0x80000000000000  => 7,
        _                                                    => 8,
    };
    for i in 0..numbytes {
        r.push ( ((x >> ((numbytes-i-1)*8)) & 0xFF) as u8 );
    }
    r
}

/*
pub struct EventsToFile<W:Write> {
   w : W,
}

impl<W:Write> EventsToFile<W> {
    pub fn new(w_ : W) -> Self {
        EventsToFile { w: w_ }
    }
    
    pub fn borrow_file    (&self)     -> &W     {     &self.w }
    pub fn borrow_file_mut(&mut self) -> &mut W { &mut self.w }
    pub fn into_file      (self)      -> W      {      self.w }
}

impl<W:Write> super::parser::EventsHandler for EventsToFile<W> {
    fn event(&mut self, e : super::parser::Event) {
        use super::parser::Event::*;

        match e {
            Begin(x)    => {
                self.w.write(&generate_big_endian_number(x.id)).unwrap();
                // Problem: we don't know what length should be here.
                // Play with offsets? (remember current offset and adjust 
                // size's size or insert Void blocks to keep up)
                x.length_including_header;
            },
            Data(ref x) => warn!("data {:?}", x),
            End(x)      => (),
            Resync      => {
                self.w.write(&generate(&el_bin(Class::Void, vec![0xFF]))).unwrap();
            },
        };
    }
}
*/
