// NONOS Operating System (AGPL-3.0-or-later)
//! Concatenate, ConcatenateResTemplate, Mid and SizeOf.

use crate::enc::*;
use crate::exec::FaultKind;
use crate::object::Object;
use crate::run::{eval, num, raw};

fn val(body: &[u8]) -> Object {
    eval(&ret(body), vec![]).unwrap()
}

#[test]
fn concatenate_takes_its_type_from_the_first_operand() {
    let ints = val(&e(0x73, &[&int(0x0201), &int(0x0403), &[0]]));
    assert!(matches!(ints, Object::Buffer(_)));
    assert_eq!(raw(&ints)[..2], [1, 2]);
    assert_eq!(raw(&ints)[8..10], [3, 4]);
    assert_eq!(raw(&ints).len(), 16);
    assert_eq!(raw(&val(&e(0x73, &[&string("AB"), &string("CD"), &[0]]))), b"ABCD");
    assert_eq!(raw(&val(&e(0x73, &[&buffer(1, &[9]), &buffer(1, &[8]), &[0]]))), [9, 8]);
    let pkg = e(0x73, &[&package(0, &[]), &int(1), &[0]]);
    assert_eq!(eval(&ret(&pkg), vec![]).unwrap_err().kind, FaultKind::Type);
}

#[test]
fn concatenate_res_template_joins_two_templates() {
    let a = buffer(5, &[0x22, 0x01, 0x00, 0x79, 0x00]);
    let b = buffer(4, &[0x47, 0x01, 0x79, 0x33]);
    let r = val(&e(0x84, &[&a, &b, &[0]]));
    assert_eq!(raw(&r), [0x22, 0x01, 0x00, 0x47, 0x01, 0x79, 0x00]);
}

#[test]
fn mid_clips_to_its_source() {
    let m = |src: &[u8], i: u64, n: u64| val(&e(0x9E, &[src, &int(i), &int(n), &[0]]));
    assert_eq!(raw(&m(&string("ABCDEF"), 1, 3)), b"BCD");
    assert_eq!(raw(&m(&string("ABCDEF"), 4, 100)), b"EF");
    assert_eq!(raw(&m(&string("ABC"), 9, 2)), b"");
    let b = m(&buffer(4, &[1, 2, 3, 4]), 2, 2);
    assert!(matches!(b, Object::Buffer(_)));
    assert_eq!(raw(&b), [3, 4]);
}

#[test]
fn sizeof_counts_characters_bytes_and_elements() {
    let body = |v: &[u8]| cat(&[&store(v, &local(0)), &ret(&e(0x87, &[&local(0)]))]);
    assert_eq!(num(&eval(&body(&string("ABCD")), vec![]).unwrap()), 4);
    assert_eq!(num(&eval(&body(&buffer(10, &[1])), vec![]).unwrap()), 10);
    assert_eq!(num(&eval(&body(&package(3, &[int(1)])), vec![]).unwrap()), 3);
    assert_eq!(eval(&body(&int(1)), vec![]).unwrap_err().kind, FaultKind::Type);
}

#[test]
fn a_buffer_is_never_shorter_than_its_initializer() {
    assert_eq!(raw(&val(&buffer(1, &[1, 2, 3]))), [1, 2, 3]);
    assert_eq!(raw(&val(&buffer(4, &[1]))), [1, 0, 0, 0]);
}
