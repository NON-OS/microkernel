// NONOS Operating System (AGPL-3.0-or-later)
//! A seeded program as AML: the mutexes at their levels, the events, and
//! `\\P` running the operations in order.

use super::gen::{Program, SyncOp};
use crate::enc::*;

pub fn mutex_name(i: usize) -> &'static str {
    ["\\M0", "\\M1", "\\M2", "\\M3", "\\_GL"][i]
}

/// The table: the mutexes, the events, and `\P` running the operations.
pub fn table(p: &Program) -> Vec<u8> {
    let mut defs: Vec<Vec<u8>> =
        (0..4).map(|i| cat(&[&op(0x5B01), &name(mutex_name(i)), &[p.levels[i]]])).collect();
    defs.extend(["\\E0", "\\E1"].map(|n| cat(&[&op(0x5B02), &name(n)])));
    let ev = |i: usize| name(["\\E0", "\\E1"][i]);
    let body: Vec<Vec<u8>> = p
        .ops
        .iter()
        .map(|o| match *o {
            SyncOp::Acquire(m, t) => e(0x5B23, &[&name(mutex_name(m)), &t.to_le_bytes()]),
            SyncOp::Release(m) => e(0x5B27, &[&name(mutex_name(m))]),
            SyncOp::Wait(v, t) => e(0x5B25, &[&ev(v), &int(t)]),
            SyncOp::Signal(v) => e(0x5B24, &[&ev(v)]),
            SyncOp::Reset(v) => e(0x5B26, &[&ev(v)]),
        })
        .collect();
    let refs: Vec<&[u8]> = body.iter().map(|b| b.as_slice()).collect();
    let mut all = defs;
    all.push(method("\\P", 0, &cat(&[&cat(&refs), &ret(&int(1))])));
    let refs: Vec<&[u8]> = all.iter().map(|b| b.as_slice()).collect();
    cat(&refs)
}
