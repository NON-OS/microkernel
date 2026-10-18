// NONOS Operating System (AGPL-3.0-or-later)
//! \_OS, \_OSI, \_REV, and load-time code run by the interpreter itself.

use crate::consts::at;
use crate::enc::*;
use crate::object::Object;
use crate::run::{boot, call, num, raw};

#[test]
fn the_predefined_objects_answer_the_default_personality() {
    let (mut aml, _) = boot(&[], 2);
    assert_eq!(raw(&call(&mut aml, "\\_OS", vec![]).unwrap()), b"Microsoft Windows NT");
    assert_eq!(num(&call(&mut aml, "\\_REV", vec![]).unwrap()), 2);
    let osi = call(&mut aml, "\\_OSI", vec![Object::string(b"Windows 2015")]).unwrap();
    assert_eq!(num(&osi), u64::MAX, "Windows 10's string is answered");
}

#[test]
fn load_time_code_runs_in_the_interpreter() {
    let src = cat(&[
        &if_(&e(0x5B12, &[&name("\\_OSI"), &[0]]), &name_obj("HASO", &int(1))),
        &if_(&cat(&[&name("\\_OSI"), &string("Linux")]), &name_obj("LNX0", &int(1))),
        &name_obj("CNT0", &int(0)),
        &store(&int(5), &name("CNT0")),
        &if_(&e(0x93, &[&name("CNT0"), &int(5)]), &device("DEV5", &[])),
    ]);
    let (aml, _) = boot(&src, 2);
    assert!(at(&aml, "\\HASO").is_some(), "CondRefOf(\\_OSI) is true");
    assert!(at(&aml, "\\LNX0").is_none(), "_OSI(\"Linux\") is false");
    assert!(at(&aml, "\\DEV5").is_some(), "a Store at load time is seen by the next If");
    assert!(aml.notes.list.is_empty(), "{:?}", aml.notes.list);
}
