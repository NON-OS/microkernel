// NONOS Operating System (AGPL-3.0-or-later)
//! Conversions, implicit (section 19.3.5.7) and explicit (19.6.137 to
//! 19.6.143). Forms the letter marks unverified are pinned here as built.

use crate::enc::*;
use crate::run::{eval, num, raw};

fn val(body: &[u8]) -> crate::object::Object {
    eval(&ret(body), vec![]).unwrap()
}

#[test]
fn implicit_operands_become_integers() {
    // String to Integer reads hex digits up to the first that is not one.
    assert_eq!(num(&val(&e(0x72, &[&string("1Fz9"), &int(1), &[0]]))), 0x20);
    // Buffer to Integer is little-endian, zero-extended.
    assert_eq!(num(&val(&e(0x72, &[&buffer(3, &[1, 2, 3]), &int(0), &[0]]))), 0x030201);
    // Only the first eight bytes count.
    let long = buffer(9, &[1, 0, 0, 0, 0, 0, 0, 0, 9]);
    assert_eq!(num(&val(&e(0x72, &[&long, &int(0), &[0]]))), 1);
}

#[test]
fn to_integer_reads_decimal_or_hex_strings() {
    assert_eq!(num(&val(&e(0x99, &[&string("1234"), &[0]]))), 1234);
    assert_eq!(num(&val(&e(0x99, &[&string("0x1F"), &[0]]))), 0x1F);
    assert_eq!(num(&val(&e(0x99, &[&string("0X10G"), &[0]]))), 16);
    assert_eq!(num(&val(&e(0x99, &[&buffer(2, &[0x34, 0x12]), &[0]]))), 0x1234);
    let over = e(0x99, &[&string("99999999999999999999999"), &[0]]);
    assert!(eval(&ret(&over), vec![]).is_err(), "a decimal past 64 bits is refused");
}

#[test]
fn explicit_text_forms() {
    assert_eq!(raw(&val(&e(0x97, &[&int(1234), &[0]]))), b"1234");
    assert_eq!(raw(&val(&e(0x98, &[&int(0x1A2B), &[0]]))), b"0x1A2B");
    assert_eq!(raw(&val(&e(0x98, &[&buffer(2, &[1, 0xAB]), &[0]]))), b"0x01,0xAB");
    assert_eq!(raw(&val(&e(0x97, &[&buffer(2, &[1, 200]), &[0]]))), b"1,200");
    assert_eq!(raw(&val(&e(0x96, &[&string("AB"), &[0]]))), b"AB\0", "ToBuffer keeps the NUL");
    assert_eq!(raw(&val(&e(0x96, &[&int(0x0102), &[0]]))), [2, 1, 0, 0, 0, 0, 0, 0]);
    let s = e(0x9C, &[&buffer(5, b"AB\0CD"), &int(u64::MAX), &[0]]);
    assert_eq!(raw(&val(&s)), b"AB", "ToString stops at the NUL");
    let s = e(0x9C, &[&buffer(4, b"ABCD"), &int(3), &[0]]);
    assert_eq!(raw(&val(&s)), b"ABC", "ToString stops at the length");
}

#[test]
fn implicit_text_forms() {
    let c = |a: &[u8], b: &[u8]| raw(&val(&e(0x73, &[a, b, &[0]])));
    assert_eq!(c(&string("v"), &int(0x1A)), b"v000000000000001A");
    assert_eq!(c(&string("b:"), &buffer(2, &[1, 0xAB])), b"b:01 AB");
}

#[test]
fn integer_width_follows_the_dsdt() {
    let (mut aml, _) = crate::run::boot(&method("M", 0, &ret(&e(0x96, &[&int(1), &[0]]))), 1);
    assert_eq!(raw(&crate::run::call(&mut aml, "\\M", vec![]).unwrap()), [1, 0, 0, 0]);
}
