use std::io::Write;

use super::parser::Event;
use super::parser::SimpleContent;
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

#[derive(Debug)]
pub enum MidlevelEvent<'a> {
    EnterElement(Class),
    LeaveElement(Class),
    
    Element(Rc<Element>), // After "Build" reply
    Content(SimpleContent<'a>),
    
    Resync,
}

pub enum WhatToDo {
    Build, // build DOM for this element
    GoOn, // continue delivering events without starting builder
}

pub trait MidlevelEventHandler {
    fn event(&mut self, e: MidlevelEvent) -> ();
    fn how_to_handle(&mut self, c: Class) -> WhatToDo;
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

        match e {
            Begin(x)    => {
                if let Some(ref mut b) = self.b {
                    b.event(e);
                } else {
                    let klass = id_to_class(x.id);
                    match self.h.how_to_handle(klass) {
                        WhatToDo::GoOn => {
                            self.h.event(MidlevelEvent::EnterElement(klass));
                        }
                        WhatToDo::Build => {
                            let mut b = Builder::new();
                            b.event(e);
                            self.b = Some(b);
                        }
                    }
                }
            }
            Data(_) if self.b.is_some() => {
                if let Some(ref mut b) = self.b {
                    b.event(e);
                }
            }
            Data(x) => {
                self.h.event(MidlevelEvent::Content(x));
            }
            End(x)      => {
                if let Some(mut b) = self.b.take() {
                    b.event(e);
                    if ! b.captured_elements().is_empty() {
                        let element = b.into_captured_elements().into_iter().next().unwrap();
                        self.h.event(MidlevelEvent::Element(element));
                    } else {
                        self.b = Some(b);
                    }
                } else {
                    self.h.event(MidlevelEvent::LeaveElement(id_to_class(x.id)));
                }
                
            }
            Resync      => {
                self.b = None;
                self.h.event(MidlevelEvent::Resync);
            },
        };
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
    fn how_to_handle(&mut self, klass: Class) -> WhatToDo {
        if klass == Class::Segment {
            WhatToDo::GoOn
        } else {
            WhatToDo::Build
        }
    }
    fn event(&mut self, e: MidlevelEvent) -> () {
        match e {
            MidlevelEvent::EnterElement(klass) => {
                if (klass == Class::Segment) {
                    self.w.write(&generate_ebml_number(class_to_id(klass), EbmlNumberMode::Identifier)).unwrap();
                    self.w.write(b"\xFF").unwrap(); // unknown length
                    self.w.write(&generate(&el_bin(Class::Void, vec![0;32]))).unwrap();
                } else {
                    panic!("Should not happen")
                }
            }
            MidlevelEvent::Element(x) => {
                self.w.write(&generate(&x)).unwrap();
            }
            MidlevelEvent::LeaveElement(_) => { 
            }
            MidlevelEvent::Content(_) => {
            }
            MidlevelEvent::Resync => {
                self.w.write(&generate(&el_bin(Class::Void, b"\nHere was resync\n".to_vec()))).unwrap();
            }
        }
    }
}
