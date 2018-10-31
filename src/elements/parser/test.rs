use super::*;
use super::debug::*;
use std::io::Write;
use std::str::from_utf8;


pub struct SerializeToStr<'a>  {
    pub s : &'a mut String,
}

impl<'a> super::EventsHandler for SerializeToStr<'a> {
    fn event(&mut self, e :super::Event) {
        use super::Event::*;
        
        let np = match e {
            Begin(x)    => format!("b {:?},", x),
            Data(ref x) => format!("d {:?},", x),
            End(x)      => format!("e {:?},", x),
            Resync      => format!("r,"),
        };
        
        self.s.push_str(&np);
    }
}


macro_rules! t {
    ($input:expr, $output:expr) => {{{
        let mut s : String = Default::default();
        {
            let mut strdumper = SerializeToStr { s:  &mut s };
            let mut p = super::new();
            p.feed_bytes(&$input, &mut strdumper);
        }
        assert_eq!(s, $output);
    }{
        let mut s : String = Default::default();
        {
            let mut strdumper = SerializeToStr { s: &mut s };
            let mut p = super::new();
            for i in $input.iter() {
                p.feed_bytes(&[*i], &mut strdumper);
            }
        }
        assert_eq!(s, $output);
    }}}
}


#[test] fn t1() {
t!( [0xA3, 0x82, 33, 44] , "b SimpleBlock(offset:0, rawlen:4),d Binary([33, 44], Full),e SimpleBlock(offset:0, rawlen:4),");}


#[test] fn t2() {
t!( [255, 255, 255, 255], "r,");}

#[test] fn t3() {
t!( [
  0x1a, 0x45, 0xdf, 0xa3, 0xa3, 0x42, 0x86, 0x81, 0x01, 0x42, 0xf7, 0x81,
  0x01, 0x42, 0xf2, 0x81, 0x04, 0x42, 0xf3, 0x81, 0x08, 0x42, 0x82, 0x88,
  0x6d, 0x61, 0x74, 0x72, 0x6f, 0x73, 0x6b, 0x61, 0x42, 0x87, 0x81, 0x02,
  0x42, 0x85, 0x81, 0x02
], 
"\
b EBML(offset:0, rawlen:40),\
b EBMLVersion(offset:5, rawlen:4),\
d Unsigned(1),\
e EBMLVersion(offset:5, rawlen:4),\
b EBMLReadVersion(offset:9, rawlen:4),\
d Unsigned(1),\
e EBMLReadVersion(offset:9, rawlen:4),\
b EBMLMaxIDLength(offset:13, rawlen:4),\
d Unsigned(4),\
e EBMLMaxIDLength(offset:13, rawlen:4),\
b EBMLMaxSizeLength(offset:17, rawlen:4),\
d Unsigned(8),\
e EBMLMaxSizeLength(offset:17, rawlen:4),\
b DocType(offset:21, rawlen:11),\
d Text(\"matroska\"),\
e DocType(offset:21, rawlen:11),\
b DocTypeVersion(offset:32, rawlen:4),\
d Unsigned(2),\
e DocTypeVersion(offset:32, rawlen:4),\
b DocTypeReadVersion(offset:36, rawlen:4),\
d Unsigned(2),\
e DocTypeReadVersion(offset:36, rawlen:4),\
e EBML(offset:0, rawlen:40),\
");}



#[test] fn t4() {
t!( [ 0xff, 0xff, 0x1a, 0x45, 0xdf, 0xa3, 0xa3], "r,b EBML(offset:2, rawlen:40),");}


#[test] fn t5() {
t!( [ 0x44, 0x89, 0x84, 0x42, 0xe8, 0x00, 0x00 ], 
        "b Duration(offset:0, rawlen:7),d Float(116.0),e Duration(offset:0, rawlen:7),");}


#[test] fn t6() {
t!( [ 0x86, 0x85, 0x41, 0x42, 0x43, 0x44, 0x45 ], 
        "b CodecID(offset:0, rawlen:7),d Text(\"ABCDE\"),e CodecID(offset:0, rawlen:7),");}
        
#[test] fn t7() {
t!( [0xA3, 0x80] , "b SimpleBlock(offset:0, rawlen:2),d Binary([], Full),e SimpleBlock(offset:0, rawlen:2),");}

#[test] fn t8() {
t!( [0x1a, 0x45, 0xdf, 0xa3, 0x80] , "b EBML(offset:0, rawlen:5),e EBML(offset:0, rawlen:5),");}
