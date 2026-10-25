// NONOS Operating System (AGPL-3.0-or-later)
//! Statements and expressions in ASL's terms, for the interpreter proofs.

use super::{cat, int, op};

/// An operator and its operands, already encoded.
pub fn e(code: u16, operands: &[&[u8]]) -> Vec<u8> {
    cat(&[&op(code), &operands.concat()])
}

pub fn ret(v: &[u8]) -> Vec<u8> {
    e(0xA4, &[v])
}

pub fn store(v: &[u8], target: &[u8]) -> Vec<u8> {
    e(0x70, &[v, target])
}

/// Return (op (a, b)) with no target.
pub fn ret_bin(code: u16, a: u64, b: u64) -> Vec<u8> {
    ret(&e(code, &[&int(a), &int(b), &[0x00]]))
}

pub fn local(n: u8) -> [u8; 1] {
    [0x60 + n]
}

pub fn arg(n: u8) -> [u8; 1] {
    [0x68 + n]
}
