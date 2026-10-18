// NONOS Operating System (AGPL-3.0-or-later)
//! A scene as AML: the mutexes, a device to notify, \M running the
//! operations, and \TRYn, which a later evaluation runs to show mutex n is
//! free: Acquire with no wait, true only if it timed out.

use super::gen::{Op, Scene};
use crate::enc::*;

fn acquire(m: &str, timeout: u16) -> Vec<u8> {
    e(0x5B23, &[&name(m), &timeout.to_le_bytes()])
}

fn release(m: &str) -> Vec<u8> {
    e(0x5B27, &[&name(m)])
}

fn op(o: &Op) -> Vec<u8> {
    let m = |n: u8| format!("\\MX{n}");
    match *o {
        Op::Acquire(n, t) => acquire(&m(n), t),
        Op::Release(n) => release(&m(n)),
        Op::AcquireGl => acquire("\\_GL", 0xFFFF),
        Op::ReleaseGl => release("\\_GL"),
        Op::Notify(v) => e(0x86, &[&name("\\_SB.DEV0"), &int(0x80 + u64::from(v))]),
        Op::Sleep(ms) => e(0x5B22, &[&int(u64::from(ms))]),
        Op::Stall(us) => e(0x5B21, &[&int(u64::from(us))]),
    }
}

pub fn source(sc: &Scene) -> Vec<u8> {
    let mut body = Vec::new();
    for (i, o) in sc.ops.iter().enumerate() {
        body.extend(op(o));
        body.extend(store(&int(i as u64), &crate::enc::op(0x5B31)));
    }
    body.extend(ret(&int(0)));
    let mut out =
        cat(&[&device("\\_SB.DEV0", &name_obj("_ADR", &int(0))), &method("\\M", 0, &body)]);
    for (n, l) in sc.levels.iter().enumerate() {
        out.extend(cat(&[&crate::enc::op(0x5B01), &name(&format!("\\MX{n}")), &[*l]]));
        out.extend(method(&format!("\\TRY{n}"), 0, &ret(&acquire(&format!("\\MX{n}"), 0))));
    }
    out
}
