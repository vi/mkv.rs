pub mod database;
pub mod parser;

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
