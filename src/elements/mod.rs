use std::rc::Rc;
use std::vec::Vec;

pub mod database;
pub mod parser;  // bytes -> events
pub mod builder; // events -> DOM
pub mod generator; // DOM -> bytes
pub mod templates;
pub mod midlevel; // events -> events+DOM

#[derive(Debug,Eq,PartialEq,Clone,Copy)]
pub enum Type {
    Master,
    Unsigned,
    Signed,
    TextAscii,
    TextUtf8,
    Binary,
    Float,
    Date,
}


// Element DOM

#[derive(PartialEq,Debug,PartialOrd,Clone)]
#[cfg_attr(feature = "rustc-serialize", derive(RustcEncodable, RustcDecodable))]
pub enum ElementContent {
    Master(Vec<Rc<Element>>),
    Unsigned(u64),
    Signed(i64),
    Binary(Rc<Vec<u8>>),
    Text(Rc<String>),
    Float(f64),
    MatroskaDate(i64), // Nanoseconds since 20010101_000000_UTC
    Unknown(u64, Rc<Vec<u8>>)
}

#[derive(PartialEq,Debug,PartialOrd,Clone)]
#[cfg_attr(feature = "rustc-serialize", derive(RustcEncodable, RustcDecodable))]
pub struct Element {
    pub class : database::Class,
    pub content : ElementContent,
}

// Cosy constructors

pub fn el_bin (c: database::Class, d:Vec<u8>) -> Element { Element { class: c, content: ElementContent::Binary  (Rc::new( d )) }} 
pub fn el_uns (c: database::Class, d:u64    ) -> Element { Element { class: c, content: ElementContent::Unsigned(         d  ) }}  
pub fn el_sig (c: database::Class, d:i64    ) -> Element { Element { class: c, content: ElementContent::Signed  (         d  ) }}  
pub fn el_flo (c: database::Class, d:f64    ) -> Element { Element { class: c, content: ElementContent::Float   (         d  ) }}  
pub fn el_txt (c: database::Class, d:String ) -> Element { Element { class: c, content: ElementContent::Text    (Rc::new( d )) }}  
pub fn el_date(c: database::Class, d:i64    ) -> Element { Element { class: c, content: ElementContent::MatroskaDate(d)}}  
pub fn el<T>(c: database::Class, d:T) -> Element   where T:IntoIterator<Item=Element> {
    let mut v = vec![];
    for i in d {
        v.push ( Rc::new(i) );
    }
    Element { class: c, content: ElementContent::Master  (v) }
}
