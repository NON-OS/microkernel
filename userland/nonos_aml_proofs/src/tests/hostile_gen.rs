// NONOS Operating System (AGPL-3.0-or-later)
//! Seeded hostile tables for F-97: byte soup, packages nested past any sane
//! depth with counts that lie, loops that run long, recursion that runs
//! deep, and buffers past the budget. Each shape that must fault says how,
//! so a bound undone shows as a wrong answer, not a hang.

use crate::enc::*;
use crate::exec::FaultKind;

pub fn next(s: &mut u64) -> u64 {
    *s ^= *s << 13;
    *s ^= *s >> 7;
    *s ^= *s << 17;
    *s
}

fn below(s: &mut u64, n: u64) -> u64 {
    next(s) % n.max(1)
}

/// Opcodes and operands at random, any length up to `most`.
pub fn soup(s: &mut u64, most: u64) -> Vec<u8> {
    (0..below(s, most)).map(|_| next(s) as u8).collect()
}

/// A package `depth` deep, its NumElements sometimes not its count.
pub fn nested_pkg(s: &mut u64, depth: u32) -> Vec<u8> {
    let n = below(s, 4) as usize;
    let mut items: Vec<Vec<u8>> = (0..n).map(|_| int(next(s) & 0xFFFF)).collect();
    if depth > 0 {
        items.push(nested_pkg(s, depth - 1));
    }
    let lie = below(s, 4) == 0;
    let count = if lie { next(s) as u8 } else { items.len() as u8 };
    package(count, &items)
}

/// What a method of each shape must end in under `Hostile::LIMITS`.
pub enum Want {
    Any,
    Fault(FaultKind),
    Value(u64),
}

/// Store (Zero, Local0); While (Local0 < n) { Increment (Local0) }.
pub fn long_loop(n: u64) -> Vec<u8> {
    let pred = e(0x95, &[&local(0), &int(n)]);
    let body = cat(&[&pkg(0xA2, &cat(&[&pred, &e(0x75, &[&local(0)])])), &ret(&local(0))]);
    cat(&[&store(&int(0), &local(0)), &body])
}

/// If (Arg0 > 0) { Return (Self (Arg0 - 1)) } Return (Zero).
pub fn recursion(seg: &str) -> Vec<u8> {
    let less = e(0x74, &[&arg(0), &int(1), &[NULL]]);
    let again = ret(&cat(&[&name(seg), &less]));
    cat(&[&if_(&e(0x94, &[&arg(0), &int(0)]), &again), &ret(&int(0))])
}

/// Return (Buffer (size) {}).
pub fn big_buffer(size: u64) -> Vec<u8> {
    ret(&buffer(size, &[]))
}
