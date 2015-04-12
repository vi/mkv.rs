use std::io::Write;

// Node: DebugPrint also used in tests

pub struct DebugPrint<'a> {
    ctr : i32,
    indent : usize,
    w: &'a mut Write,
}

pub fn debug_logger<'a>(w : &'a mut Write) -> DebugPrint<'a> {
    DebugPrint { ctr: 0, indent: 0, w: w }
}

impl<'a> super::EventsHandler for DebugPrint<'a> {
    fn event(&mut self, e :super::Event) {
        use super::Event::*;
        
        match e {
            End(_) => if self.indent > 0 { self.indent -= 1 },
            _ => (),
        }
        for _ in 0..self.indent { write!(self.w, " "); }
        match e {
            Begin(x)    => writeln!(self.w, "element {:?}", x),
            Data(ref x) => writeln!(self.w, "data {:?}", x),
            End(x)      => writeln!(self.w, "end {:?}", x),
            Resync      => writeln!(self.w, "resync"),
        };
        match e {
            Begin(_) => self.indent += 1,
            _ => (),
        }
        
        self.ctr+=1;
    }
    fn log(&mut self, t : &str) {
        println!("log: {}", t);
    }
}
