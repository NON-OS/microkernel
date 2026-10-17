// NONOS Operating System (AGPL-3.0-or-later)
//! Mutexes: nesting, sync level order, and one another evaluation left held.

use crate::consts::at;
use crate::enc::*;
use crate::exec::{FaultKind, Holder};
use crate::run::{boot, call, call_with, num, Probe};

fn mutex(n: &str, level: u8) -> Vec<u8> {
    cat(&[&op(0x5B01), &name(n), &[level]])
}

fn acquire(n: &str, timeout: u16) -> Vec<u8> {
    e(0x5B23, &[&name(n), &timeout.to_le_bytes()])
}

#[path = "cut_sync/mod.rs"]
mod cut_sync;

#[test]
fn mutexes_nest_and_keep_their_sync_order() {
    let defs = cat(&[&mutex("MLO", 1), &mutex("MHI", 5)]);
    let ok = cat(&[
        &acquire("MLO", 0xFFFF),
        &acquire("MLO", 0xFFFF),
        &acquire("MHI", 0),
        &e(0x5B27, &[&name("MHI")]),
        &e(0x5B27, &[&name("MLO")]),
        &e(0x5B27, &[&name("MLO")]),
        &ret(&int(7)),
    ]);
    let bad = cat(&[&acquire("MHI", 0xFFFF), &acquire("MLO", 0xFFFF)]);
    let unheld = e(0x5B27, &[&name("MLO")]);
    let (mut aml, _) = boot(
        &cat(&[
            &defs,
            &method("\\OK", 0, &ok),
            &method("\\BAD", 0, &bad),
            &method("\\UNH", 0, &unheld),
        ]),
        2,
    );
    assert_eq!(num(&call(&mut aml, "\\OK", vec![]).unwrap()), 7);
    assert_eq!(call(&mut aml, "\\BAD", vec![]).unwrap_err().kind, FaultKind::Mutex);
    assert!(aml.rt.mutexes.is_empty(), "a faulted evaluation releases what it held");
    assert_eq!(call(&mut aml, "\\UNH", vec![]).unwrap_err().kind, FaultKind::Mutex);
}

#[test]
fn a_mutex_another_evaluation_left_held_times_out() {
    let defs = mutex("MUT0", 0);
    let (mut aml, _) = boot(
        &cat(&[
            &defs,
            &method("\\T", 0, &ret(&acquire("MUT0", 16))),
            &method("\\F", 0, &ret(&acquire("MUT0", 0xFFFF))),
        ]),
        2,
    );
    let m = at(&aml, "\\MUT0").unwrap();
    aml.rt.mutexes.insert(m, Holder { eval: u64::MAX, depth: 1 });
    let mut probe = Probe::default();
    let r = call_with(&mut aml, &mut probe, Default::default(), "\\T", vec![]).unwrap();
    assert_eq!(num(&r), u64::MAX, "timed out");
    assert_eq!(probe.slept_ms, [16]);
    assert_eq!(
        call(&mut aml, "\\F", vec![]).unwrap_err().kind,
        FaultKind::Mutex,
        "for ever cannot end"
    );
}

#[path = "sync_sweep/mod.rs"]
mod sweep;
