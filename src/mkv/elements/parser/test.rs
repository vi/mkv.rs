use super::super::Event;
use super::super::EventsHandler;


/*struct GatherEvents<'a> {
    accumulator : Vec<Event<'a> >,
}

impl<'a> EventsHandler for GatherEvents<'a> {
    fn event(&mut self, e : Event){
        self.accumulator.push(e);
    }
    fn log(&mut self, m : &str) { println!("{}",m)  }
}*/

#[test]
fn element_parse_test_1() {
    let b = &[255];
    
}
