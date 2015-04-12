use super::*;

macro_rules! ehi {
    ($x:ty, $ctr:ident, $e:ident, $c:block) => {
        impl<'a> EventsHandler for $x {
            fn event(&mut self, $e : Event) {*self.$ctr += 1; $c }
            fn log(&mut self, m : &str) { println!("{}", m); }
        }
    }
    
}
    
#[test]
fn element_parse_test_1() {
    let mut ctr = 0;
    struct Q<'a> { ctr: &'a mut i32};
    ehi!(Q<'a>, ctr, e, {
        match e {
            Event::Resync => (),
            _ => panic!("Expected resync"),
        }
    });
    {
        let mut p = super::new(Q{ctr : &mut ctr});
        p.feed_bytes(&[255, 255, 255, 255]);
    }
    assert_eq!(ctr, 1);
}
