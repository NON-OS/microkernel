// NONOS Operating System (AGPL-3.0-or-later)
//! A PCI_Config region's function, and _REG run when a space connects.

use crate::enc::*;
use crate::exec::PciPath;
use crate::rig::Rig;

#[path = "reg/mod.rs"]
mod reg;

#[test]
fn a_pci_config_region_knows_its_function() {
    let lpc = device(
        "LPCB",
        &cat(&[
            &name_obj("_ADR", &int(0x001F_0000)),
            &region("PCFG", 2, 0x40, 0x10),
            &field("PCFG", 1, &[unit("VAL", 8)]),
            &method("RD", 0, &ret(&name("VAL"))),
        ]),
    );
    let host = device(
        "PCI0",
        &cat(&[
            &name_obj("_HID", &int(0x080A_D041)),
            &name_obj("_SEG", &int(1)),
            &name_obj("_BBN", &int(0x10)),
            &lpc,
        ]),
    );
    let mut r = Rig::new(&scope("\\_SB", &host)).connected();
    r.call("\\_SB.PCI0.LPCB.RD", vec![]).unwrap();
    assert_eq!(r.probe.pci, [PciPath { segment: 1, bus: 0x10, adrs: vec![0x001F_0000] }]);
    assert_eq!(r.probe.log[0].1, 0x40);
}

#[test]
fn reg_runs_where_regions_of_the_space_are() {
    let ec = device(
        "EC0",
        &cat(&[
            &region("ECR", 3, 0, 0x100),
            &name_obj("ECOK", &int(0)),
            &method("_REG", 2, &if_(&e(0x93, &[&arg(0), &int(3)]), &store(&arg(1), &name("ECOK")))),
        ]),
    );
    let other = device(
        "OTH",
        &cat(&[&name_obj("CALL", &int(0)), &method("_REG", 2, &store(&int(1), &name("CALL")))]),
    );
    let mut r = Rig::new(&scope("\\_SB", &cat(&[&ec, &other])));
    assert_eq!(r.num("\\_SB.EC0.ECOK"), 0);
    assert!(r.connect(3).is_empty());
    assert_eq!(r.num("\\_SB.EC0.ECOK"), 1);
    assert_eq!(r.num("\\_SB.OTH.CALL"), 0, "a device with no region of the space is not told");
}

#[test]
fn a_reg_that_faults_is_reported_and_the_rest_still_run() {
    let bad = device("BAD", &cat(&[&region("R1", 3, 0, 1), &method("_REG", 2, &ret(&local(0)))]));
    let good = device(
        "GOOD",
        &cat(&[
            &region("R2", 3, 0, 1),
            &name_obj("DONE", &int(0)),
            &method("_REG", 2, &store(&int(1), &name("DONE"))),
        ]),
    );
    let mut r = Rig::new(&cat(&[&bad, &good]));
    let failed = r.connect(3);
    assert_eq!(failed.len(), 1);
    assert_eq!(r.num("\\GOOD.DONE"), 1);
}
