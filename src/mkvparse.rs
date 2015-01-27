use std::vec::Vec;
use std::string::String;

pub trait MkvCallbacks {
    fn debug(&self, str : String);
}

pub trait MkvParser<Cb : MkvCallbacks> {
    fn initialize(cb : Cb) -> Self;
    fn feed_bytes(&mut self, bytes : Vec<u8>);
}

pub struct State<Cb> {
    cb : Cb,
    accumulator : Vec<u8>,
}

impl<Cb:MkvCallbacks> MkvParser<Cb> for State<Cb> {
    fn initialize(cb : Cb) -> State<Cb> {
        State {
            accumulator: vec![],
            cb : cb,
        }
    }
    
    fn feed_bytes(&mut self, bytes : Vec<u8>)
    {
        self.cb.debug(String::from_str("Hello, world"));
    }
}
