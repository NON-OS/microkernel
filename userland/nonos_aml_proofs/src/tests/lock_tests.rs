// NONOS Operating System (AGPL-3.0-or-later)
//! The Global Lock: taken around a Lock field and by Acquire(\_GL), waited
//! for while firmware holds it, and handed back with GBL_RLS when firmware
//! was waiting.

use crate::enc::*;
use crate::probe::Probe;
use crate::rig::Rig;

fn locked_field(probe: Probe) -> Rig {
    let defs = cat(&[&region("REG0", 1, 0x60, 1), &field("REG0", 0x11, &[unit("LCK", 8)])]);
    Rig::with(&cat(&[&defs, &method("\\M", 0, &store(&int(1), &name("LCK")))]), probe).connected()
}

#[test]
fn a_lock_field_takes_and_gives_back_the_lock() {
    let mut r = locked_field(Probe { facs: Some(0), ..Default::default() });
    r.call("\\M", vec![]).unwrap();
    assert_eq!(r.probe.facs, Some(0));
    assert_eq!(r.aml.rt.global_lock, 0);
    assert_eq!(r.probe.released, 0);
}

#[test]
fn firmware_holding_the_lock_is_waited_for() {
    let mut r = locked_field(Probe { facs: Some(2), firmware_holds: 3, ..Default::default() });
    r.call("\\M", vec![]).unwrap();
    assert!(!r.probe.slept_ms.is_empty(), "waited while firmware held it");
    assert_eq!(r.probe.facs, Some(0));
}

#[test]
fn giving_back_a_lock_firmware_waits_for_sets_gbl_rls() {
    let body = cat(&[&e(0x5B23, &[&name("\\_GL"), &[0xFF, 0xFF]]), &e(0x5B27, &[&name("\\_GL")])]);
    let mut r = Rig::with(
        &method("\\B", 0, &body),
        Probe { facs: Some(0), firmware_waits: true, ..Default::default() },
    );
    r.call("\\B", vec![]).unwrap();
    assert_eq!(r.probe.facs, Some(0));
    assert_eq!(r.probe.released, 1);
}

#[test]
fn an_evaluation_that_ends_holding_gl_gives_the_lock_back() {
    let body = cat(&[&e(0x5B23, &[&name("\\_GL"), &[0xFF, 0xFF]]), &ret(&local(0))]);
    let mut r = Rig::with(&method("\\B", 0, &body), Probe { facs: Some(0), ..Default::default() });
    assert!(r.call("\\B", vec![]).is_err());
    assert_eq!((r.probe.facs, r.aml.rt.global_lock), (Some(0), 0));
}

#[test]
fn without_a_facs_the_lock_is_ours_at_once() {
    let mut r = locked_field(Probe::default());
    r.call("\\M", vec![]).unwrap();
    assert!(r.probe.slept_ms.is_empty());
}
