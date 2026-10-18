// NONOS Operating System (AGPL-3.0-or-later)
//! Packages and buffers: elements may be data or names that refer, and a
//! package may nest packages to the depth limit.

use crate::arena::Imm;
use crate::enc::*;
use crate::error::ErrorKind;
use crate::flat::{parse, parse_with, Flat};
use crate::op::Op;

#[test]
fn elements_are_data_or_references() {
    let prt = package(4, &[int(0xFFFF), int(0), name("^LNKA"), int(0)]);
    let aml = name_obj("PRT0", &package(1, &[prt]));
    let (arena, root) = parse(&aml).unwrap();
    let outer = arena.kids(arena.kids(root)[0])[1];
    let entry = arena.kids(arena.kids(outer)[1])[0];
    let elems = arena.kids(arena.kids(entry)[1]);
    let ops: Vec<Op> = elems.iter().map(|k| arena.node(*k).unwrap().op).collect();
    assert_eq!(ops, [Op::WORD_CONST, Op::ZERO, Op::NAME_STRING, Op::ZERO]);
}

#[test]
fn a_name_in_a_package_is_never_called() {
    let mut flat = Flat::default();
    flat.methods.insert(*b"MTH1", 1);
    let aml = name_obj("PKG0", &package(2, &[name("MTH1"), int(1)]));
    let (arena, root) = parse_with(&aml, &mut flat, Default::default()).unwrap();
    let pkg = arena.kids(arena.kids(root)[0])[1];
    assert_eq!(arena.kids(arena.kids(pkg)[1]).len(), 2);
}

#[test]
fn statements_and_locals_are_not_elements() {
    let err = parse(&name_obj("PKG0", &package(1, &[vec![0xA3]]))).unwrap_err();
    assert_eq!(err.kind, ErrorKind::Misplaced(0xA3));
    let err = parse(&name_obj("PKG0", &package(1, &[vec![LOCAL0]]))).unwrap_err();
    assert_eq!(err.kind, ErrorKind::Misplaced(0x60));
}

#[test]
fn a_buffer_keeps_its_initializer_as_bytes() {
    let (arena, root) = parse(&name_obj("BUF0", &buffer(8, &[0x47, 0x01, 0x79, 0x00]))).unwrap();
    let buf = arena.kids(arena.kids(root)[0])[1];
    let init = arena.node(arena.kids(buf)[1]).unwrap();
    assert_eq!(init.op, Op::BYTES);
    let Imm::Bytes(s) = init.imm else { panic!("bytes") };
    assert_eq!(s.len, 4);
}

#[test]
fn a_package_whose_length_overruns_its_name_is_refused() {
    let mut aml = name_obj("PKG0", &package(1, &[int(1)]));
    let len_at = 1 + 4 + 1;
    aml[len_at] += 1;
    assert_eq!(parse(&aml).unwrap_err().kind, ErrorKind::PastParent);
}
