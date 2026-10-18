// NONOS Operating System (AGPL-3.0-or-later)
//! The logical operators: True is Ones, the second operand converts to the
//! first's type, strings and buffers compare byte by byte.

use crate::enc::*;
use crate::object::Object;
use crate::run::{eval, num};

fn logic(code: u16, a: &[u8], b: &[u8]) -> u64 {
    num(&eval(&ret(&e(code, &[a, b])), vec![]).unwrap())
}

#[test]
fn integer_comparisons_and_connectives() {
    let t = u64::MAX;
    assert_eq!(logic(0x93, &int(5), &int(5)), t);
    assert_eq!(logic(0x93, &int(5), &int(6)), 0);
    assert_eq!(logic(0x94, &int(6), &int(5)), t);
    assert_eq!(logic(0x95, &int(5), &int(6)), t);
    assert_eq!(logic(0x90, &int(2), &int(3)), t, "LAnd of two non-zero values");
    assert_eq!(logic(0x90, &int(2), &int(0)), 0);
    assert_eq!(logic(0x91, &int(0), &int(7)), t);
    assert_eq!(logic(0x91, &int(0), &int(0)), 0);
    let not = |v: u64| num(&eval(&ret(&e(0x92, &[&int(v)])), vec![]).unwrap());
    assert_eq!((not(0), not(9)), (t, 0));
    // LNotEqual, LLessEqual, LGreaterEqual are LNot around the others.
    let ne = ret(&e(0x92, &[&e(0x93, &[&int(1), &int(2)])]));
    assert_eq!(num(&eval(&ne, vec![]).unwrap()), t);
}

#[test]
fn strings_and_buffers_compare_byte_by_byte() {
    assert_eq!(logic(0x93, &string("ABC"), &string("ABC")), u64::MAX);
    assert_eq!(logic(0x95, &string("AB"), &string("ABC")), u64::MAX, "a prefix is less");
    assert_eq!(logic(0x94, &string("B"), &string("ABC")), u64::MAX);
    assert_eq!(logic(0x93, &buffer(2, &[1, 2]), &buffer(2, &[1, 2])), u64::MAX);
    assert_eq!(logic(0x95, &buffer(2, &[1, 2]), &buffer(2, &[1, 3])), u64::MAX);
}

#[test]
fn the_second_operand_converts_to_the_first_type() {
    assert_eq!(logic(0x93, &int(0x1A), &string("1A")), u64::MAX, "the string read as hex");
    assert_eq!(logic(0x93, &int(0x0201), &buffer(2, &[1, 2])), u64::MAX);
    assert_eq!(logic(0x93, &string("0000000000000010"), &int(0x10)), u64::MAX);
}

#[test]
fn a_package_cannot_be_compared() {
    let body = ret(&e(0x93, &[&package(1, &[int(1)]), &int(1)]));
    assert!(eval(&body, vec![]).is_err());
    let _ = Object::Integer(0);
}
