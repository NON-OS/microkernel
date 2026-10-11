// NONOS Operating System (AGPL-3.0-or-later)
//! Store and CopyObject: a Name keeps its type and a buffer its length, a
//! local takes a copy, and an argument holding a reference is written
//! through.

use crate::enc::*;
use crate::run::{boot, call, num, raw};

fn run(defs: &[u8], body: &[u8]) -> crate::object::Object {
    let (mut aml, _) = boot(&cat(&[defs, &method("\\M", 0, body)]), 2);
    call(&mut aml, "\\M", vec![]).unwrap()
}

#[test]
fn a_store_to_a_name_converts_to_its_type() {
    let int0 = name_obj("INT0", &int(5));
    assert_eq!(
        num(&run(&int0, &cat(&[&store(&string("1F"), &name("INT0")), &ret(&name("INT0"))]))),
        0x1F
    );
    let str0 = name_obj("STR0", &string("x"));
    let r = run(&str0, &cat(&[&store(&int(0xAB), &name("STR0")), &ret(&name("STR0"))]));
    assert_eq!(raw(&r), b"00000000000000AB");
}

#[test]
fn a_named_buffer_keeps_its_length() {
    let buf0 = name_obj("BUF0", &buffer(4, &[1, 2, 3, 4]));
    let short =
        run(&buf0, &cat(&[&store(&buffer(2, &[9, 9]), &name("BUF0")), &ret(&name("BUF0"))]));
    assert_eq!(raw(&short), [9, 9, 0, 0]);
    let long = run(&buf0, &cat(&[&store(&buffer(6, &[7; 6]), &name("BUF0")), &ret(&name("BUF0"))]));
    assert_eq!(raw(&long), [7, 7, 7, 7]);
}

#[test]
fn copy_object_replaces_the_type() {
    let int0 = name_obj("INT0", &int(5));
    let r = run(&int0, &cat(&[&e(0x9D, &[&string("abc"), &name("INT0")]), &ret(&name("INT0"))]));
    assert_eq!(raw(&r), b"abc");
}

#[test]
fn a_local_takes_a_copy_and_index_writes_in_place() {
    let buf0 = name_obj("BUF0", &buffer(2, &[1, 2]));
    let idx = |src: &[u8], i: u64| e(0x88, &[src, &int(i), &[0]]);
    let body = cat(&[
        &store(&name("BUF0"), &local(0)),
        &store(&int(9), &idx(&local(0), 0)),
        &ret(&e(0x83, &[&idx(&name("BUF0"), 0)])),
    ]);
    assert_eq!(num(&run(&buf0, &body)), 1, "the copy changed, not the Name");
    let pkg0 = name_obj("PKG0", &package(2, &[int(1), int(2)]));
    let body =
        cat(&[&store(&int(7), &idx(&name("PKG0"), 1)), &ret(&e(0x83, &[&idx(&name("PKG0"), 1)]))]);
    assert_eq!(num(&run(&pkg0, &body)), 7);
}

#[test]
fn an_argument_holding_a_reference_is_written_through() {
    let set = method("SET", 1, &store(&int(42), &arg(0)));
    let body =
        cat(&[&store(&int(0), &local(0)), &name("SET"), &e(0x71, &[&local(0)]), &ret(&local(0))]);
    assert_eq!(num(&run(&set, &body)), 42);
    let body = cat(&[&store(&int(3), &local(0)), &name("SET"), &local(0), &ret(&local(0))]);
    assert_eq!(num(&run(&set, &body)), 3, "an argument passed by value leaves the caller's local");
}
