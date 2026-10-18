// NONOS Operating System (AGPL-3.0-or-later)
//! A field touched before its _REG. The specification asks firmware not to
//! touch a region before _REG(space, 1) (6.5.4; recalled, unverified); it
//! does not say what OSPM does when it does. This interpreter neither
//! defers nor queues: with no handler for the space it refuses (NoHandler);
//! with the handler connected it runs the access, whether or not that
//! region's own scope has had its _REG yet.

use super::gen::{reg_body, said};
use crate::enc::*;
use crate::exec::FaultKind;
use crate::probe::Probe;
use crate::rig::Rig;

const EC: u8 = 3;

fn ec_device(path: &str, base: u64, unit_name: &str, reg: &[u8]) -> Vec<u8> {
    let r = cat(&[&region("ECR", EC, base, 1), &field("ECR", 0x01, &[unit(unit_name, 8)])]);
    device(path, &cat(&[&r, &method("_REG", 2, reg)]))
}

#[test]
fn before_the_handler_it_is_refused() {
    let defs = cat(&[
        &ec_device("\\D0", 0, "F0", &reg_body(0)),
        &method("\\RD", 0, &ret(&name("\\D0.F0"))),
    ]);
    let mut r = Rig::new(&defs);
    assert_eq!(r.call("\\RD", vec![]).unwrap_err().kind, FaultKind::NoHandler);
    assert!(r.probe.log.is_empty() && r.probe.debug.is_empty(), "nothing reached, nothing said");
    assert!(r.connect(EC).is_empty());
    assert_eq!(r.probe.debug, [said(0, EC, 1)]);
    assert_eq!(crate::run::num(&r.call("\\RD", vec![]).unwrap()), 0, "runs once connected");
}

#[test]
fn inside_another_reg_it_runs_before_its_own() {
    // D0's _REG reads D1's field while D1 is still waiting for its _REG.
    let peek = cat(&[&store(&name("\\D1.F1"), &op(0x5B31)), &reg_body(0)]);
    let defs =
        cat(&[&ec_device("\\D0", 0, "F0", &peek), &ec_device("\\D1", 8, "F1", &reg_body(1))]);
    let mut r = Rig::with(&defs, Probe::default());
    r.probe.bytes.insert((EC, 8), 0x5A);
    assert!(r.connect(EC).is_empty());
    assert_eq!(r.probe.debug, ["Integer(90)".to_string(), said(0, EC, 1), said(1, EC, 1)]);
    assert_eq!(r.probe.log, [(EC, 8, 1, None)]);
}

#[test]
fn a_reg_touching_a_space_not_yet_connected_faults_and_the_rest_run() {
    let io = cat(&[&region("IOR", 1, 0x80, 1), &field("IOR", 0x01, &[unit("IOF", 8)])]);
    let touch = cat(&[&reg_body(0), &store(&int(1), &name("\\IOF"))]);
    let defs =
        cat(&[&io, &ec_device("\\D0", 0, "F0", &touch), &ec_device("\\D1", 8, "F1", &reg_body(1))]);
    let mut r = Rig::new(&defs);
    let failed = r.connect(EC);
    assert_eq!(failed.iter().map(|(_, f)| f.kind).collect::<Vec<_>>(), [FaultKind::NoHandler]);
    assert_eq!(r.probe.debug, [said(0, EC, 1), said(1, EC, 1)]);
    assert!(r.probe.log.is_empty());
}
