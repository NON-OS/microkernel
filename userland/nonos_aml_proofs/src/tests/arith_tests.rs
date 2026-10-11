// NONOS Operating System (AGPL-3.0-or-later)
//! The integer operators with two operands against the results the
//! specification gives, at 64 and at 32 bits.

use crate::enc::*;
use crate::exec::FaultKind;
use crate::run::{boot, call, eval, num};

fn bin(code: u16, a: u64, b: u64) -> u64 {
    num(&eval(&ret_bin(code, a, b), vec![]).unwrap())
}

#[test]
fn binary_operators_wrap_at_64_bits() {
    let max = u64::MAX;
    assert_eq!(bin(0x72, max, 2), 1, "Add wraps");
    assert_eq!(bin(0x74, 1, 2), max, "Subtract wraps");
    assert_eq!(bin(0x77, 1 << 63, 2), 0, "Multiply wraps");
    assert_eq!(bin(0x7B, 0xF0F0, 0xFF00), 0xF000);
    assert_eq!(bin(0x7C, 0xF0F0, 0xFF00), !0xF000);
    assert_eq!(bin(0x7D, 0xF0F0, 0x0F0F), 0xFFFF);
    assert_eq!(bin(0x7E, 0xF0F0, 0x0F0F), !0xFFFF);
    assert_eq!(bin(0x7F, 0xFF00, 0x0FF0), 0xF0F0);
    assert_eq!(bin(0x85, 17, 5), 2, "Mod");
    assert_eq!(bin(0x79, 1, 63), 1 << 63);
    assert_eq!(bin(0x79, 1, 64), 0, "a shift by the width leaves nothing");
    assert_eq!(bin(0x7A, 1 << 63, 63), 1);
    assert_eq!(bin(0x7A, max, 200), 0);
}

#[test]
fn a_revision_1_table_computes_in_32_bits() {
    let m = method("M", 0, &ret_bin(0x72, 0xFFFF_FFFF, 2));
    let n = method("N", 0, &ret(&[0xFF]));
    let s = method("S", 0, &ret_bin(0x79, 1, 32));
    let (mut aml, _) = boot(&cat(&[&m, &n, &s]), 1);
    assert_eq!(num(&call(&mut aml, "\\M", vec![]).unwrap()), 1);
    assert_eq!(num(&call(&mut aml, "\\N", vec![]).unwrap()), 0xFFFF_FFFF, "Ones is 32 bits");
    assert_eq!(num(&call(&mut aml, "\\S", vec![]).unwrap()), 0);
}

#[test]
fn divide_gives_the_remainder_and_the_quotient() {
    let body = cat(&[
        &e(0x78, &[&int(17), &int(5), &local(0), &local(1)]),
        &ret(&e(0x72, &[&local(0), &e(0x77, &[&local(1), &int(100), &[0]]), &[0]])),
    ]);
    assert_eq!(num(&eval(&body, vec![]).unwrap()), 2 + 300);
    for code in [0x78, 0x85] {
        let (a, b, t) = (int(1), int(0), [0u8]);
        let all: [&[u8]; 4] = [&a, &b, &t, &t];
        let zero = ret(&e(code, &all[..if code == 0x78 { 4 } else { 3 }]));
        assert_eq!(eval(&zero, vec![]).unwrap_err().kind, FaultKind::DivideByZero);
    }
}
