// NONOS Operating System (AGPL-3.0-or-later)
//! The integer operators with one operand, and Increment and Decrement.

use crate::enc::*;
use crate::exec::FaultKind;
use crate::run::{eval, num};

#[test]
fn unary_operators() {
    let un = |code: u16, v: u64| num(&eval(&ret(&e(code, &[&int(v), &[0]])), vec![]).unwrap());
    assert_eq!(un(0x80, 0), u64::MAX, "Not");
    assert_eq!(un(0x81, 0), 0, "FindSetLeftBit of zero");
    assert_eq!(un(0x81, 0x80), 8);
    assert_eq!(un(0x81, 1 << 63), 64);
    assert_eq!(un(0x82, 0x80), 8, "FindSetRightBit");
    assert_eq!(un(0x82, 0x81), 1);
    assert_eq!(un(0x5B29, 1234), 0x1234, "ToBCD");
    assert_eq!(un(0x5B28, 0x1234), 1234, "FromBCD");
    let bad = ret(&e(0x5B28, &[&int(0x1A), &[0]]));
    assert_eq!(eval(&bad, vec![]).unwrap_err().kind, FaultKind::Type);
}

#[test]
fn increment_and_decrement_store_back() {
    let body = cat(&[
        &store(&int(41), &local(0)),
        &e(0x75, &[&local(0)]),
        &e(0x75, &[&local(0)]),
        &e(0x76, &[&local(0)]),
        &ret(&local(0)),
    ]);
    assert_eq!(num(&eval(&body, vec![]).unwrap()), 42);
    let wrap = cat(&[&store(&int(0), &local(0)), &ret(&e(0x76, &[&local(0)]))]);
    assert_eq!(num(&eval(&wrap, vec![]).unwrap()), u64::MAX);
}
