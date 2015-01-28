use mkv::EventsHandler;
use mkv::Parser;
use mkv::AuxilaryEvent::Debug;


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
