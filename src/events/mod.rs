/// MidlevelEvents -> MatroskaEvents

extern crate bytes;

use super::elements::midlevel::{MidlevelEventHandler,WhatToDo,MidlevelEvent};
use super::elements::database::Class;
use super::elements::parser::{SimpleContent,BinaryChunkStatus};

use bytes::{Buf,IntoBuf,BigEndian};

/// TODO: revise this Vec<Vec<u8>>
#[derive(Debug, Eq, PartialEq, Hash)]
pub struct MatroskaFrame {
    pub timecode_nanoseconds: u64,
    pub track_number: usize,
    /// outer vec - lacing, inner vec - byte buffer
    pub buffers: Vec<Vec<u8>>,
}

/// TODO: more functions (or rewrite the whole thing)
pub trait MatroskaEventHandler {
    fn frame_encountered(&mut self, f: MatroskaFrame);
    fn segment_tracks(&mut self, e: &std::rc::Rc<super::elements::Element>);
    fn segment_info(&mut self, e: &std::rc::Rc<super::elements::Element>) { }
}

enum WhatToAwait {
    Block,
    TimecodeScale,
    ClusterTimecode,
    Nothing,
}

pub struct MatroskaDemuxer<H: MatroskaEventHandler> {
    h: H,
    timecode_scale: u64,
    last_cluster_timecode: u64,
    a: WhatToAwait,
    b: Vec<u8>,
}


impl<H: MatroskaEventHandler> MidlevelEventHandler for MatroskaDemuxer<H> {
  fn event(&mut self, e: MidlevelEvent) -> () {
    match e {
        MidlevelEvent::EnterElement(c) => {
            self.b.clear();
            match c {
                Class::SimpleBlock => self.a = WhatToAwait::Block,
                Class::Block => self.a = WhatToAwait::Block,
                Class::TimecodeScale => self.a = WhatToAwait::TimecodeScale,
                Class::Timecode => self.a = WhatToAwait::ClusterTimecode,
                _ => {self.a = WhatToAwait::Nothing},
            }
        },
        MidlevelEvent::Content(x) => {
            match self.a {
                WhatToAwait::TimecodeScale => {
                    match x {
                        SimpleContent::Unsigned(xx) => {
                            self.timecode_scale = xx;
                            info!("Got timecode scale: {}", xx);
                        }
                        _ => { error!("Unexpected type"); }
                    }
                }
                WhatToAwait::ClusterTimecode => {
                    match x {
                        SimpleContent::Unsigned(xx) => {
                            self.last_cluster_timecode = xx;
                            info!("Got cluster timecode: {}", xx);
                        }
                        _ => { error!("Unexpected type"); }
                    }
                }
                WhatToAwait::Block => {
                    match x {
                        SimpleContent::Binary(xx, status) => {
                            self.b.extend(xx);
                            match status {
                                BinaryChunkStatus::Last | BinaryChunkStatus::Full => {
                                    self.process_frame();
                                }
                                _ => {},
                            }
                        }
                        _ => { error!("Unexpected type"); }
                    }
                }
                _ => {},
            }
        },
        MidlevelEvent::LeaveElement(_) => {},
        MidlevelEvent::Element(e) => {
            match e.class {
                Class::Info => self.h.segment_info(&e),
                Class::Tracks => self.h.segment_tracks(&e),
                _ => warn!("This code line should not be reachable"),
            }
        },
        MidlevelEvent::Resync => {
            warn!("Resynging matroska parser")
            // TODO: report to user
        },
    }
  }
  fn how_to_handle(&mut self, c: Class) -> WhatToDo {
    match c {
        Class::Info => WhatToDo::Build,
        Class::Tracks => WhatToDo::Build,
        _ => WhatToDo::GoOn,
    }
  }
}

impl<H: MatroskaEventHandler> MatroskaDemuxer<H> {
    pub fn new(h: H) -> Self { 
        MatroskaDemuxer {
            h,
            timecode_scale: 1000_000,
            last_cluster_timecode: 0,
            a: WhatToAwait::Nothing,
            b: vec![],
        }
    }
    
    fn process_frame(&mut self) {
        let mut b  = (&self.b).into_buf();
        
        let tn_ = b.get_u8();
        if tn_ & 0x80 != 0x80 {
            error!("large track numbers not supported");
            return;
        }
        let tn = tn_ & 0x7F;
        
        let ts = b.get_i16::<BigEndian>();
        
        let flags = b.get_u8();
        
        if flags&0x06 != 0x00 {
            error!("lacing not supported yet");
        }
        
        // TODO: extract more info from flags and report it to user
        
        // TODO: to it properly
        let mut ts_nano = ((self.last_cluster_timecode as i64) + (ts as i64)) as u64;
        ts_nano = ((ts_nano as f64) * (self.timecode_scale as f64)) as u64;
        
        let remaining: Vec<u8> = b.collect();
        
        let f = MatroskaFrame {
            timecode_nanoseconds : ts_nano,
            track_number: tn as usize,
            buffers: vec![remaining],
        };
        
        self.h.frame_encountered(f);
    }
}
