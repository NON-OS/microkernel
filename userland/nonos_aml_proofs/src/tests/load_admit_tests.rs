// NONOS Operating System (AGPL-3.0-or-later)
//! F-93: every table AML loads at run time is offered to the host first,
//! as the exact bytes that then load, by Load from a buffer or a region and
//! by LoadTable; a refusal loads nothing and answers as a failed load.

use crate::consts::at;
use crate::enc::*;
use crate::object::Object;
use crate::probe::Probe;
use crate::rig::Rig;

fn ssdt() -> Vec<u8> {
    table(b"SSDT", 2, &scope("\\_SB", &name_obj("NEW0", &int(9))))
}

fn by_buffer() -> Vec<u8> {
    let defs = name_obj("TBL0", &buffer(ssdt().len() as u64, &ssdt()));
    let body = ret(&e(0x5B20, &[&name("TBL0"), &local(0)]));
    cat(&[&defs, &method("\\L", 0, &body)])
}

fn by_table() -> Vec<u8> {
    let s = |t: &str| string(t);
    let lt = e(0x5B1F, &[&s("SSDT"), &s(""), &s(""), &s("\\"), &s(""), &int(0)]);
    method("\\L", 0, &ret(&lt))
}

#[test]
fn the_bytes_offered_are_the_bytes_loaded() {
    let mut r = Rig::new(&by_buffer());
    assert_eq!(r.num("\\L"), u64::MAX);
    assert_eq!(r.probe.admitted, [ssdt()], "Load offers its buffer, once");
    assert_eq!(r.num("\\_SB.NEW0"), 9);
    let mut r = Rig::with(&by_table(), Probe { tables: vec![ssdt()], ..Default::default() });
    assert!(matches!(r.call("\\L", vec![]).unwrap(), Object::Table(_)));
    assert_eq!(r.probe.admitted, [ssdt()], "LoadTable offers the table it found");
    assert_eq!(r.num("\\_SB.NEW0"), 9);
}

#[test]
fn a_refused_table_never_loads() {
    let mut r = Rig::with(&by_buffer(), Probe { refuse_tables: true, ..Default::default() });
    assert_eq!(r.num("\\L"), 0, "Load answers Zero");
    assert!(at(&r.aml, "\\_SB.NEW0").is_none());
    let refuse = Probe { tables: vec![ssdt()], refuse_tables: true, ..Default::default() };
    let mut r = Rig::with(&by_table(), refuse);
    assert_eq!(r.num("\\L"), 0, "LoadTable answers Zero");
    assert!(at(&r.aml, "\\_SB.NEW0").is_none());
}
