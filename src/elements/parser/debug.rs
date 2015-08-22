use std::io::Write;

use std::iter;

#[derive(Default)]
pub struct DebugPrint {
    ctr : i32,
    indent : usize,
}

impl super::EventsHandler for DebugPrint {
    fn event(&mut self, e :super::Event) {
        use super::Event::*;
        
        match e {
            End(_) => if self.indent > 0 { self.indent -= 1 },
            _ => (),
        }
        let indentstr : String =  iter::repeat(" ").take(self.indent).collect();
        match e {
            Begin(x)    => debug!("{}element {:?}", indentstr, x),
            Data(ref x) => debug!("{}data {:?}", indentstr, x),
            End(x)      => debug!("{}end {:?}", indentstr, x),
            Resync      => debug!("{}resync", indentstr),
        };
        match e {
            Begin(_) => self.indent += 1,
            _ => (),
        }
        
        self.ctr+=1;
    }
}
