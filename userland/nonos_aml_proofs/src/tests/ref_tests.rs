// NONOS Operating System (AGPL-3.0-or-later)
//! RefOf, DerefOf, CondRefOf, Index and ObjectType.

use crate::enc::*;
use crate::exec::FaultKind;
use crate::run::{boot, call, num};

fn run(defs: &[u8], body: &[u8]) -> Result<crate::object::Object, crate::exec::Fault> {
    let (mut aml, _) = boot(&cat(&[defs, &method("\\M", 0, body)]), 2);
    call(&mut aml, "\\M", vec![])
}

#[test]
fn cond_ref_of_tells_present_from_absent() {
    let int0 = name_obj("INT0", &int(5));
    let found = cat(&[&ret(&e(0x5B12, &[&name("INT0"), &local(0)]))]);
    assert_eq!(num(&run(&int0, &found).unwrap()), u64::MAX);
    let deref = cat(&[&e(0x5B12, &[&name("INT0"), &local(0)]), &ret(&e(0x83, &[&local(0)]))]);
    assert_eq!(num(&run(&int0, &deref).unwrap()), 5);
    let absent = ret(&e(0x5B12, &[&name("\\NONE"), &local(0)]));
    assert_eq!(num(&run(&int0, &absent).unwrap()), 0);
}

#[test]
fn deref_of_a_string_reads_the_path_it_names() {
    let defs = device("\\_SB.DEV0", &name_obj("VAL0", &int(0x77)));
    let body = ret(&e(0x83, &[&string("\\_SB.DEV0.VAL0")]));
    assert_eq!(num(&run(&defs, &body).unwrap()), 0x77);
    let bad = ret(&e(0x83, &[&string("\\_SB.NOPE")]));
    assert_eq!(run(&defs, &bad).unwrap_err().kind, FaultKind::Undefined);
}

#[test]
fn a_reference_to_a_name_writes_to_it() {
    let int0 = name_obj("INT0", &int(5));
    let body = cat(&[&store(&int(9), &e(0x71, &[&name("INT0")])), &ret(&name("INT0"))]);
    assert_eq!(num(&run(&int0, &body).unwrap()), 9);
}

#[test]
fn index_past_the_end_is_a_fault() {
    let pkg0 = name_obj("PKG0", &package(2, &[int(1), int(2)]));
    let body = ret(&e(0x88, &[&name("PKG0"), &int(2), &[0]]));
    assert_eq!(run(&pkg0, &body).unwrap_err().kind, FaultKind::Index);
    let body = ret(&e(0x88, &[&int(5), &int(0), &[0]]));
    assert_eq!(run(&pkg0, &body).unwrap_err().kind, FaultKind::Type);
}

#[test]
fn object_type_numbers_every_kind() {
    let defs = cat(&[
        &device("DEV0", &[]),
        &method("MTH0", 0, &[0xA3]),
        &cat(&[&op(0x5B01), &name("MUT0"), &[0]]),
        &cat(&[&op(0x5B02), &name("EVT0")]),
        &cat(&[&op(0x5B80), &name("REG0"), &[0], &int(0), &int(4)]),
        &name_obj("STR0", &string("s")),
        &name_obj("PKG0", &package(1, &[int(1)])),
    ]);
    let ty = |n: &str| num(&run(&defs, &ret(&e(0x8E, &[&name(n)]))).unwrap());
    assert_eq!(ty("DEV0"), 6);
    assert_eq!(ty("MTH0"), 8);
    assert_eq!(ty("MUT0"), 9);
    assert_eq!(ty("EVT0"), 7);
    assert_eq!(ty("REG0"), 10);
    assert_eq!(ty("STR0"), 2);
    assert_eq!(ty("PKG0"), 4);
    let local = cat(&[&store(&buffer(1, &[]), &local(0)), &ret(&e(0x8E, &[&local(0)]))]);
    assert_eq!(num(&run(&defs, &local).unwrap()), 3);
}
