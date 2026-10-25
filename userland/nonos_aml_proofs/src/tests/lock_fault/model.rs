// NONOS Operating System (AGPL-3.0-or-later)
//! The model the sweep holds each run to (`sweep_tests.rs` states it).

use super::gen::{Fault, Scene};
use crate::exec::FaultKind;
use crate::rig::Rig;

pub fn uncut(sc: &Scene) -> Option<FaultKind> {
    match (sc.holds, sc.fault) {
        (Some(u32::MAX), _) => Some(FaultKind::Time),
        (_, Fault::Select) => Some(FaultKind::NoHandler),
        (_, Fault::Data) => Some(FaultKind::Index),
        _ => None,
    }
}

pub fn given_back(r: &Rig, sc: &Scene, what: &str) {
    assert_eq!(r.aml.rt.global_lock, 0, "{what}: count");
    match r.probe.facs {
        Some(w) if r.probe.firmware_holds > 0 => assert!(w & 2 != 0, "{what}: firmware's"),
        Some(w) => assert_eq!(w & 3, 0, "{what}: word {w:#x}"),
        None => {}
    }
    if !sc.waits {
        assert_eq!(r.probe.released, 0, "{what}: GBL_RLS unasked");
    }
}
