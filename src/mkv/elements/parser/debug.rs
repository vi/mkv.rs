use std::io::Write;

pub struct DebugPrint<W : Write> {
    ctr : i32,
    indent : usize,
    w: W,
}

pub fn debug_logger<W : Write>(w : W) -> DebugPrint<W> {
    DebugPrint { ctr: 0, indent: 0, w: w }
}

impl<W:Write> super::EventsHandler for DebugPrint<W> {
    fn event(&mut self, e :super::Event) {
        use super::Event::*;
        
        match e {
            End(_) => if self.indent > 0 { self.indent -= 1 },
            _ => (),
        }
        for _ in 0..self.indent { write!(&mut self.w, " "); }
        match e {
            Begin(x)    => writeln!(&mut self.w, "element {:?}", x),
            Data(ref x) => writeln!(&mut self.w, "data {:?}", x),
            End(x)      => writeln!(&mut self.w, "end {:?}", x),
            Resync      => writeln!(&mut self.w, "resync"),
        };
        match e {
            Begin(_) => self.indent += 1,
            _ => (),
        }
        
        self.ctr+=1;
    }
    fn log(&mut self, t : &str) {
        write!(self.w, "log: {}", t);
    }
}
