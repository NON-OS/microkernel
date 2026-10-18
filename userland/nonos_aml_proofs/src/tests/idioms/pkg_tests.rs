// NONOS Operating System (AGPL-3.0-or-later)
//! Packages as firmware passes them around, as the specification's Index
//! and DerefOf examples use them (ACPI 6.5, 19.6.63 and 19.6.32; recalled,
//! unverified): a method's returned package indexed in place, an element
//! of a nested package written through Index on DerefOf(Index(...)), which
//! reaches the package the outer one holds, and a package element passed
//! as a method's argument.

use super::more::*;
use crate::enc::*;
use crate::object::Object;
use crate::rig::Rig;

fn at(p: &[u8], i: u64) -> Vec<u8> {
    e(0x88, &[p, &int(i), &[0]])
}

fn deref(p: &[u8]) -> Vec<u8> {
    e(0x83, &[p])
}

fn rig() -> Rig {
    let inner = |a: u64, b: u64| package(2, &[int(a), int(b)]);
    let sum = bin(ADD, &deref(&at(&arg(0), 0)), &deref(&at(&arg(0), 1)));
    let defs = cat(&[
        &name_obj("\\PK", &package(3, &[inner(1, 2), inner(3, 4), int(5)])),
        &method("\\GP", 0, &ret(&name("\\PK"))),
        &method("\\TWO", 1, &ret(&sum)),
        &method("\\SUM", 1, &ret(&invoke("\\TWO", &[deref(&at(&name("\\GP"), 0))]))),
        &method("\\SET", 2, &store(&arg(1), &at(&deref(&at(&name("\\PK"), 1)), 0))),
    ]);
    Rig::new(&defs).connected()
}

fn call(r: &mut Rig, path: &str, args: &[u64]) -> u64 {
    let args = args.iter().map(|v| Object::Integer(*v)).collect();
    crate::run::num(&r.call(path, args).expect("runs"))
}

#[test]
fn nested_elements_read_written_and_passed() {
    let mut r = rig();
    assert_eq!(call(&mut r, "\\SUM", &[0]), 3, "the inner package as an argument");
    r.call("\\SET", vec![Object::Integer(1), Object::Integer(9)]).expect("the write runs");
    let inner = |r: &mut Rig, i: usize| match r.call("\\PK", vec![]).unwrap() {
        Object::Package(p) => match &p.borrow()[i] {
            Object::Package(q) => q.borrow().iter().map(crate::run::num).collect::<Vec<_>>(),
            o => panic!("{o:?}"),
        },
        o => panic!("{o:?}"),
    };
    assert_eq!(inner(&mut r, 1), [9, 4], "the write reached the package \\PK holds");
    assert_eq!(inner(&mut r, 0), [1, 2], "and no other");
}
