use super::*;
use super::debug::*;
use std::io::Write;
use std::str::from_utf8;
    
macro_rules! t {
    ($input:expr, $output:expr) => {{{
        let mut out = vec!();
        {
            let mut p = super::new( debug_logger(&mut out as &mut Write)  );
            p.feed_bytes(&$input);
        }
        assert_eq!(from_utf8(out.as_slice()), Ok($output));
    }{
        let mut out = vec!();
        {
            let mut p = super::new( debug_logger(&mut out as &mut Write)  );
            for i in $input.iter() {
                p.feed_bytes(&[*i]);
            }
        }
        assert_eq!(from_utf8(out.as_slice()), Ok($output));
    }}}
}


#[test] fn t1() {
t!( [0xA3, 0x82, 33, 44] ,
r#"element SimpleBlock(offset:0, rawlen:4)
 data Binary([33, 44])
end SimpleBlock(offset:0, rawlen:4)
"#);}


#[test] fn t2() {
t!( [255, 255, 255, 255], 
r#"resync
"#);}

#[test] fn t3() {
t!( [
  0x1a, 0x45, 0xdf, 0xa3, 0xa3, 0x42, 0x86, 0x81, 0x01, 0x42, 0xf7, 0x81,
  0x01, 0x42, 0xf2, 0x81, 0x04, 0x42, 0xf3, 0x81, 0x08, 0x42, 0x82, 0x88,
  0x6d, 0x61, 0x74, 0x72, 0x6f, 0x73, 0x6b, 0x61, 0x42, 0x87, 0x81, 0x02,
  0x42, 0x85, 0x81, 0x02
], 
r#"element EBML(offset:0, rawlen:40)
 element EBMLVersion(offset:5, rawlen:4)
  data Unsigned(1)
 end EBMLVersion(offset:5, rawlen:4)
 element EBMLReadVersion(offset:9, rawlen:4)
  data Unsigned(1)
 end EBMLReadVersion(offset:9, rawlen:4)
 element EBMLMaxIDLength(offset:13, rawlen:4)
  data Unsigned(4)
 end EBMLMaxIDLength(offset:13, rawlen:4)
 element EBMLMaxSizeLength(offset:17, rawlen:4)
  data Unsigned(8)
 end EBMLMaxSizeLength(offset:17, rawlen:4)
 element DocType(offset:21, rawlen:11)
  data Text("matroska")
 end DocType(offset:21, rawlen:11)
 element DocTypeVersion(offset:32, rawlen:4)
  data Unsigned(2)
 end DocTypeVersion(offset:32, rawlen:4)
 element DocTypeReadVersion(offset:36, rawlen:4)
  data Unsigned(2)
 end DocTypeReadVersion(offset:36, rawlen:4)
end EBML(offset:0, rawlen:40)
"#);}



#[test] fn t4() {
t!( [ 0xff, 0xff, 0x1a, 0x45, 0xdf, 0xa3, 0xa3], 
r#"resync
element EBML(offset:2, rawlen:40)
"#);}


#[test] fn t5() {
t!( [ 0x44, 0x89, 0x84, 0x42, 0xe8, 0x00, 0x00 ],
r#"element Duration(offset:0, rawlen:7)
 data Float(116)
end Duration(offset:0, rawlen:7)
"#);}



#[test] fn t6() {
t!( [ 0x86, 0x85, 0x41, 0x42, 0x43, 0x44, 0x45 ],
r#"element CodecID(offset:0, rawlen:7)
 data Text("ABCDE")
end CodecID(offset:0, rawlen:7)
"#);}
