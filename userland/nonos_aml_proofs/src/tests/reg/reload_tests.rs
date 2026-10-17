// NONOS Operating System (AGPL-3.0-or-later)
//! A table unloaded and loaded again: its devices are new objects, so their
//! _REG runs again, even where the namespace reuses the old slots.

use super::gen::{reg_body, said};
use crate::enc::*;
use crate::probe::Probe;
use crate::rig::Rig;

#[test]
fn a_reloaded_table_is_announced_again() {
    let r0 = cat(&[&region("ECR", 3, 0, 1), &method("_REG", 2, &reg_body(7))]);
    let t = table(b"SSDT", 2, &device("\\D07", &r0));
    let load = cat(&[&e(0x5B20, &[&name("\\TB"), &local(0)]), &ret(&local(0))]);
    let defs = cat(&[
        &name_obj("\\TB", &buffer(t.len() as u64, &t)),
        &method("\\L", 0, &load),
        &method("\\U", 1, &e(0x5B2A, &[&arg(0)])),
    ]);
    let mut r = Rig::with(&defs, Probe::default());
    assert!(r.connect(3).is_empty());
    for round in 1..=3 {
        let handle = r.call("\\L", vec![]).expect("loads");
        assert_eq!(r.probe.debug, vec![said(7, 3, 1); round], "round {round}");
        r.call("\\U", vec![handle]).expect("unloads");
    }
}

#[test]
fn a_reg_that_loads_a_table_is_not_run_again_from_inside() {
    // D0's _REG loads a table whose D1 has an EC region of its own: D1 is
    // told from inside the load, D0 only the once.
    let r1 = cat(&[&region("ECR", 3, 8, 1), &method("_REG", 2, &reg_body(1))]);
    let t = table(b"SSDT", 2, &device("\\D01", &r1));
    let load = cat(&[&e(0x5B20, &[&name("\\TB"), &local(0)]), &reg_body(0)]);
    let r0 = cat(&[&region("ECR", 3, 0, 1), &method("_REG", 2, &load)]);
    let defs = cat(&[&name_obj("\\TB", &buffer(t.len() as u64, &t)), &device("\\D00", &r0)]);
    let mut r = Rig::with(&defs, Probe::default());
    assert!(r.connect(3).is_empty());
    assert_eq!(r.probe.debug, [said(1, 3, 1), said(0, 3, 1)]);
}

#[test]
fn a_reg_fault_after_a_load_is_counted_and_the_load_kept() {
    let bad = cat(&[&region("ECR", 3, 0, 1), &method("_REG", 2, &ret(&local(0)))]);
    let good = cat(&[&region("ECR", 3, 8, 1), &method("_REG", 2, &reg_body(2))]);
    let t = table(b"SSDT", 2, &cat(&[&device("\\D01", &bad), &device("\\D02", &good)]));
    let load = cat(&[&e(0x5B20, &[&name("\\TB"), &local(0)]), &ret(&local(0))]);
    let defs = cat(&[
        &name_obj("\\TB", &buffer(t.len() as u64, &t)),
        &device("\\D00", &bad),
        &method("\\L", 0, &load),
    ]);
    let mut r = Rig::with(&defs, Probe::default());
    assert_eq!(r.connect(3).len(), 1, "a fault at connect is returned");
    assert_eq!(r.aml.rt.reg_faults, 0, "and not counted again");
    let got = r.call("\\L", vec![]).expect("the load is kept");
    assert!(matches!(got, crate::object::Object::Table(_)));
    assert_eq!(r.aml.rt.reg_faults, 1, "D01's fault, counted");
    assert_eq!(r.probe.debug, [said(2, 3, 1)], "D02 still told");
}
