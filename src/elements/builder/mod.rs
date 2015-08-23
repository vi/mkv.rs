use std::rc::Rc;
use std::cell::RefCell;
use super::Element;
use super::ElementContent;
use super::parser::SimpleContent;
use super::parser::EventsHandler;
use super::parser::Event;

#[cfg(test)] mod test;

#[derive(Default)]
pub struct Builder {
    ce : Vec<Rc<Element>>, // finished elements
    nb : Option<Box<RefCell<Builder>>>,
    cc : Option<ElementContent>,
    cid : u64,
    in_middle : bool,
}

impl Builder {
    pub fn captured_elements(&self) -> &Vec<Rc<Element>> {
        &self.ce 
    }
}

impl EventsHandler for Builder {
    fn event(&mut self, e : Event) {
        use super::parser::Event::*;

        if let Some(nb) = self.nb.take() {
            let terminated = match &e {
                &End(_) if !nb.borrow().in_middle => true,
                _ => false,
            };
            if terminated {
                self.cc = Some(ElementContent::Master(nb.borrow_mut().ce.clone()));
            } else {
                nb.borrow_mut().event(e);
                self.nb = Some(nb);
                return;
            }
        }
        match e {
            Begin(x)    => {
                self.in_middle = true;
                self.cid = x.id;
                
                let cl = super::database::id_to_class(x.id);
                let typ = super::database::class_to_type(cl);
        
                if typ == super::Type::Master {
                    assert!(self.nb.is_none());
                    self.nb = Some ( Box::new( RefCell::new(Default::default()) ) );
                };
            },
            Data(ref x) => {
                self.cc = Some(match (*x) {
                    SimpleContent::Unsigned(x) => ElementContent::Unsigned(x),
                    SimpleContent::Signed(x) => ElementContent::Signed(x),
                    SimpleContent::Text(x) => ElementContent::Text(Rc::new(x.to_string())),
                    SimpleContent::Binary(x) => ElementContent::Binary(Rc::new(x.to_vec())),
                    SimpleContent::Float(x) => ElementContent::Float(x),
                    SimpleContent::MatroskaDate(x) => ElementContent::MatroskaDate(x),
                });
            },
            End(x)      => { 
                let cl = super::database::id_to_class(self.cid);
                let elc = self.cc.take().unwrap();
                let el = Element {class: cl, content: elc};
                self.ce.push(Rc::new(el));
                self.in_middle = false;
            },
            Resync      => debug!("resync"),
        };
    }
}
