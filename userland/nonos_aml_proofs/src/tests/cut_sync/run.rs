// NONOS Operating System (AGPL-3.0-or-later)
//! One run of a scene under a step limit, what it left, and the checks
//! every run must pass (`sweep_tests.rs` states them).

use super::fixture::source;
use super::gen::Scene;
use super::model::Snap;
use crate::exec::{ExecLimits, FaultKind, Machine};
use crate::probe::Probe;
use crate::rig::Rig;

pub fn once(sc: &Scene, steps: u64) -> (Option<FaultKind>, u64, Rig, u32) {
    let facs = sc.holds.map(|h| if h > 0 { 2 } else { 0 });
    let probe = Probe {
        facs,
        firmware_holds: sc.holds.unwrap_or(0),
        firmware_waits: sc.waits,
        ..Default::default()
    };
    let mut r = Rig::with(&source(sc), probe);
    let id = crate::consts::at(&r.aml, "\\M").unwrap();
    let lim = ExecLimits { steps, time_ms: 200, ..ExecLimits::DEFAULT };
    let mut m = Machine::new(&mut r.aml, &mut r.probe, lim);
    let got = m.evaluate(&mut r.aml, id, vec![]).err().map(|f| f.kind);
    let used = m.steps();
    let left = r.aml.rt.left_held;
    (got, used, r, left)
}

pub fn seen(r: &Rig, left: u32) -> Snap {
    Snap {
        notes: r.aml.rt.notifications.len(),
        held: left as usize,
        // Waits for the Global Lock sleep 1 ms; the scenes' Sleeps 10 or more.
        sleeps: r.probe.slept_ms.iter().filter(|ms| **ms >= 10).count(),
        stalls: r.probe.stalled_us.len(),
    }
}

pub fn given_back(r: &mut Rig, sc: &Scene, what: &str) {
    assert!(r.aml.rt.mutexes.is_empty() && r.aml.rt.global_lock == 0, "{what}: held");
    match r.probe.facs {
        Some(w) if r.probe.firmware_holds > 0 => assert!(w & 2 != 0, "{what}: firmware's"),
        Some(w) => assert_eq!(w & 3, 0, "{what}: word {w:#x}"),
        None => {}
    }
    assert!(sc.waits || r.probe.released == 0, "{what}: GBL_RLS unasked");
    for n in 0..3 {
        assert_eq!(r.num(&format!("\\TRY{n}")), 0, "{what}: MX{n} free for the next");
    }
}
