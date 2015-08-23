use super::*;
use super::super::parser::Info;
use super::super::parser::Event;
use super::super::parser::SimpleContent;
use super::super::parser::EventsHandler;
use super::super::database::Class;
use super::super::database::class_to_id;
use super::super::*;
use std::rc::Rc;


#[test] fn t1() {
    let mut b : Builder = Default::default();
    assert_eq!(*b.captured_elements(), vec![]);
}

#[test] fn t2() {
    let mut b : Builder = Default::default();
    b.event(Event::Begin(&Info { id : 0xA3, offset: 0, length_including_header: Some(4) }));
    b.event(Event::Data(SimpleContent::Binary(&vec![0x44, 0x55])));
    b.event(Event::End(&Info { id : 0xA3, offset: 0, length_including_header: Some(4) }));
    assert_eq!(*b.captured_elements(), vec![
        Rc::new(el_bin(Class::SimpleBlock, vec![0x44, 0x55])),
    ]);
}


#[test] fn t3() {
    let mut b : Builder = Default::default();
    
    b.event(Event::Begin(&Info { id : 0xCC, offset: 0, length_including_header: Some(3) }));
    b.event(Event::Data(SimpleContent::Unsigned(33)));
    b.event(Event::End(&Info { id : 0xCC, offset: 0, length_including_header: Some(3) }));
    
    b.event(Event::Begin(&Info { id : 0xFB, offset: 3, length_including_header: Some(3) }));
    b.event(Event::Data(SimpleContent::Signed(-33)));
    b.event(Event::End(&Info { id : 0xFB, offset: 3, length_including_header: Some(3) }));
    
    assert_eq!(*b.captured_elements(), vec![
        Rc::new(el_uns(Class::LaceNumber, 33)),
        Rc::new(el_sig(Class::ReferenceBlock, -33)),
    ]);
}


#[test] fn t4() {
    let mut b : Builder = Default::default();
    
    b.event(Event::Begin(&Info { id : 0x1F43B675, offset: 0, length_including_header: Some(5) }));
    b.event(Event::End(&Info { id : 0x1F43B675, offset: 0, length_including_header: Some(5) }));
    
    assert_eq!(*b.captured_elements(), vec![
        Rc::new(el(Class::Cluster, vec![])),
    ]);
}


#[test] fn t5() {
    let mut b : Builder = Default::default();
    
    b.event(Event::Begin(&Info { id : 0x1F43B675, offset: 0, length_including_header: Some(8) }));
     b.event(Event::Begin(&Info { id : 0xE7, offset: 5, length_including_header: Some(3) }));
     b.event(Event::Data(SimpleContent::Unsigned(100)));
     b.event(Event::End(&Info { id : 0xE7, offset: 5, length_including_header: Some(3) }));
    b.event(Event::End(&Info { id : 0x1F43B675, offset: 0, length_including_header: Some(8) }));
    
    assert_eq!(*b.captured_elements(), vec![
        Rc::new(el(Class::Cluster, vec![
                el_uns(Class::Timecode, 100),
        ])),
    ]);
}

#[test] fn t6() {
    let mut b : Builder = Default::default();
    
    b.event(Event::Begin(&Info { id : class_to_id(Class::EBML), offset: 0, length_including_header: Some(40) }));
     b.event(Event::Begin(&Info { id : class_to_id(Class::EBMLVersion), offset: 5, length_including_header: Some(4) }));
      b.event(Event::Data (SimpleContent::Unsigned(1)));
     b.event(Event::End  (&Info { id : class_to_id(Class::EBMLVersion), offset: 5, length_including_header: Some(4) }));
     
     b.event(Event::Begin(&Info { id : class_to_id(Class::EBMLReadVersion), offset: 9, length_including_header: Some(4) }));
      b.event(Event::Data (SimpleContent::Unsigned(1)));
     b.event(Event::End  (&Info { id : class_to_id(Class::EBMLReadVersion), offset: 9, length_including_header: Some(4) }));
     
     b.event(Event::Begin(&Info { id : class_to_id(Class::EBMLMaxIDLength), offset: 13, length_including_header: Some(4) }));
      b.event(Event::Data (SimpleContent::Unsigned(4)));
     b.event(Event::End  (&Info { id : class_to_id(Class::EBMLMaxIDLength), offset: 13, length_including_header: Some(4) }));
     
     b.event(Event::Begin(&Info { id : class_to_id(Class::EBMLMaxSizeLength), offset: 17, length_including_header: Some(4) }));
      b.event(Event::Data (SimpleContent::Unsigned(8)));
     b.event(Event::End  (&Info { id : class_to_id(Class::EBMLMaxSizeLength), offset: 17, length_including_header: Some(4) }));
     
     b.event(Event::Begin(&Info { id : class_to_id(Class::DocType), offset: 21, length_including_header: Some(11) }));
      b.event(Event::Data (SimpleContent::Text("matroska")));
     b.event(Event::End  (&Info { id : class_to_id(Class::DocType), offset: 21, length_including_header: Some(11) }));
     
     b.event(Event::Begin(&Info { id : class_to_id(Class::DocTypeVersion), offset: 32, length_including_header: Some(4) }));
      b.event(Event::Data (SimpleContent::Unsigned(2)));
     b.event(Event::End  (&Info { id : class_to_id(Class::DocTypeVersion), offset: 32, length_including_header: Some(4) }));
     
     b.event(Event::Begin(&Info { id : class_to_id(Class::DocTypeReadVersion), offset: 36, length_including_header: Some(4) }));
      b.event(Event::Data (SimpleContent::Unsigned(2)));
     b.event(Event::End  (&Info { id : class_to_id(Class::DocTypeReadVersion), offset: 36, length_including_header: Some(4) }));
     
    b.event(Event::End  (&Info { id : class_to_id(Class::EBML), offset: 0, length_including_header: Some(40) }));
    
    assert_eq!(*b.captured_elements(), vec![
        Rc::new(super::super::templates::ebml_header(false)),
    ]);
}



