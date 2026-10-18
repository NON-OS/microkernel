// NONOS Operating System (AGPL-3.0-or-later)
//! The sweep: each seeded program run by the interpreter and by the model.
//! After every run, whatever ended: the fault (or none), the sleeps in
//! order, the events' counts, the mutexes left held counted, nothing of
//! this evaluation still held, and the Global Lock given back.

use super::gen::{program, Program};
use super::model::{run, Outcome};
use super::table::{mutex_name, table};
use crate::consts::at;
use crate::exec::{ExecLimits, FaultKind, Holder};
use crate::run::{boot, call_with, Probe};

fn interpret(p: &Program) -> (Outcome, usize) {
    let (mut aml, _) = boot(&table(p), 2);
    for (i, _) in p.foreign.iter().enumerate().filter(|(_, f)| **f) {
        let m = at(&aml, mutex_name(i)).unwrap();
        aml.rt.mutexes.insert(m, Holder { eval: u64::MAX, depth: 1 });
    }
    let before = aml.rt.left_held;
    let mut probe = Probe::default();
    let limits = ExecLimits { time_ms: p.time_ms, ..ExecLimits::DEFAULT };
    let fault = call_with(&mut aml, &mut probe, limits, "\\P", Vec::new()).err().map(|f| f.kind);
    let events = ["\\E0", "\\E1"].map(|n| {
        let e = at(&aml, n).unwrap();
        aml.rt.events.get(&e).copied().unwrap_or(0)
    });
    assert_eq!(aml.rt.global_lock, 0, "the Global Lock given back");
    let left = aml.rt.left_held - before;
    (Outcome { fault, slept: probe.slept_ms, events, left }, aml.rt.mutexes.len())
}

#[test]
fn hostile_sync_is_its_model() {
    let (mut clean, mut mutex, mut time, mut left, mut over) = (0, 0, 0, 0, 0);
    for seed in 1..=3000u64 {
        let p = program(&mut seed.wrapping_mul(0x9E37_79B9_7F4A_7C15));
        let want = run(&p);
        let (got, still) = interpret(&p);
        assert_eq!(
            got, want,
            "seed {seed}: {:?} levels {:?} foreign {:?}",
            p.ops, p.levels, p.foreign
        );
        assert_eq!(still, p.foreign.iter().filter(|f| **f).count(), "seed {seed}: only another's");
        over += usize::from(want.fault == Some(FaultKind::Time) && !want.slept.is_empty());
        match want.fault {
            None => clean += 1,
            Some(FaultKind::Mutex) => mutex += 1,
            Some(_) => time += 1,
        }
        left += usize::from(want.left > 0);
    }
    let says = format!("{clean} clean, {mutex} mutex faults, {time} time faults, {left} left held");
    let enough = clean >= 300 && mutex >= 300 && time >= 150 && left >= 300 && over >= 30;
    assert!(enough, "{says}, {over} past the budget after sleeping");
}
