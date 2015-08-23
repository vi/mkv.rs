use std::io::Write;

use std::iter;

pub struct DebugPrint {
    ctr : i32,
    indent : usize,
    ll : ::log::LogLevel,
}

impl DebugPrint {
    pub fn new(log_level : ::log::LogLevel) -> Self {
        DebugPrint { ctr: 0, indent: 0, ll: log_level }
    }
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
            Begin(x)    => log!(self.ll, "{}element {:?}", indentstr, x),
            Data(ref x) => log!(self.ll, "{}data {:?}", indentstr, x),
            End(x)      => log!(self.ll, "{}end {:?}", indentstr, x),
            Resync      => log!(self.ll, "{}resync", indentstr),
        };
        match e {
            Begin(_) => self.indent += 1,
            _ => (),
        }
        
        self.ctr+=1;
    }
}
