use std::io::Write;

use super::parser::Event;
use super::database::Class;
use super::database::id_to_class;
use super::database::class_to_id;
use super::generator::generate_ebml_number;
use super::generator::EbmlNumberMode;
use std::rc::Rc;
use super::Element;
use super::parser::EventsHandler;
use super::builder::Builder;
use super::generator::generate;
use super::el_bin;

pub enum MidlevelEvent {
    EnterElement(Class),
    LeaveElement(Class),
    Element(Rc<Element>),
    Resync,
}

#[derive(PartialEq)]
pub enum WhatToDo {
    Build, // build DOM for this element
    GoOn, // go to next element
}

pub trait MidlevelEventHandler {
    fn handle(&mut self, e: MidlevelEvent) -> WhatToDo;
}

/// Mid-evel parser allows using Builder, but skipping headers of Segments or Clusters
/// to avoid holding too much data in memory;
pub struct MidlevelParser<H:MidlevelEventHandler> {
    h:H,
    b:Option<Builder>
}

impl<H:MidlevelEventHandler> MidlevelParser<H> {
    pub fn new(h_:H) -> MidlevelParser<H> {
        MidlevelParser { h:h_ , b:None}
    }
    
    pub fn borrow_handler    (&self)     -> &H {        &self.h }
    pub fn borrow_handler_mut(&mut self) -> &mut H {&mut self.h }
    pub fn into_handler      (self)      -> H { self.h }
}

impl<H:MidlevelEventHandler> EventsHandler for MidlevelParser<H> {
    fn event(&mut self, e : super::parser::Event) {
        use super::parser::Event::*;

        let mut choice = WhatToDo::GoOn;
        match e {
            Begin(x)    => {
                if let Some(ref mut b) = self.b {
                    b.event(e);
                } else {
                    let choice2 = self.h.handle(MidlevelEvent::EnterElement(id_to_class(x.id)));
                    match choice2 {
                        WhatToDo::GoOn => (),
                        WhatToDo::Build => {
                            let mut b = Builder::new();
                            b.event(e);
                            self.b = Some(b);
                        }
                    }
                }
                ;
            }
            Data(_) => {
                if let Some(ref mut b) = self.b {
                    b.event(e);
                } else {
                    // ignore it
                }
            }
            End(x)      => {
                if let Some(mut b) = self.b.take() {
                    b.event(e);
                    if ! b.captured_elements().is_empty() {
                        let element = b.into_captured_elements().into_iter().next().unwrap();
                        choice = self.h.handle(MidlevelEvent::Element(element));
                    } else {
                        self.b = Some(b);
                    }
                } else {
                    choice = self.h.handle(MidlevelEvent::LeaveElement(id_to_class(x.id)));
                }
                
            }
            Resync      => {
                self.b = None;
                choice = self.h.handle(MidlevelEvent::Resync);
            },
        };
        if choice == WhatToDo::Build {
            self.b = Some(Builder::new());
        }
    }
}


/// Serialize events back into mkv file.
/// Segment size will be unfilled, Void will be auto-insterted.
/// EBML head will be copied as is
pub struct MidlevelEventsToFile<W:Write> {
    w: W,
}

impl<W:Write> MidlevelEventsToFile<W> {
    pub fn new(w_:W) -> Self {
        MidlevelEventsToFile { w:w_ }
    }
    
    pub fn borrow_file    (&self)     -> &W {        &self.w }
    pub fn borrow_file_mut(&mut self) -> &mut W {&mut self.w }
    pub fn into_file      (self)      -> W { self.w }
}

impl<W:Write> MidlevelEventHandler for MidlevelEventsToFile<W> {
    fn handle(&mut self, e: MidlevelEvent) -> WhatToDo {
        match e {
            MidlevelEvent::EnterElement(klass) => {
                if (klass == Class::Segment) {
                    self.w.write(&generate_ebml_number(class_to_id(klass), EbmlNumberMode::Identifier)).unwrap();
                    self.w.write(b"\xFF").unwrap(); // unknown length
                    self.w.write(&generate(&el_bin(Class::Void, vec![0;32]))).unwrap();
                    WhatToDo::GoOn
                } else {
                    WhatToDo::Build
                }
            }
            MidlevelEvent::Element(x) => {
                self.w.write(&generate(&x)).unwrap();
                WhatToDo::GoOn
            }
            MidlevelEvent::LeaveElement(_) => { 
                WhatToDo::GoOn
            }
            MidlevelEvent::Resync => {
                self.w.write(&generate(&el_bin(Class::Void, b"\nHere was resync\n".to_vec()))).unwrap();
                WhatToDo::GoOn
            }
        }
    }
}
