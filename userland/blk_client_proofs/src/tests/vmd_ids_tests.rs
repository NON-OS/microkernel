// NONOS Operating System (AGPL-3.0-or-later)
//! The disk list names every VMD the kernel brings up, and 28C1, which the
//! kernel leaves hidden: on any of them a person with no disk in the list
//! is told to switch the firmware's storage mode, never shown nothing.

#[path = "../../../../src/drivers/pci/vmd/domain/ids.rs"]
mod kernel_vmd;

use crate::driver::{classify, Controller, INTEL_VMD};

#[test]
fn every_vmd_the_kernel_knows_is_named_in_the_disk_list() {
    for id in kernel_vmd::VMD_DEVICE_IDS {
        assert!(kernel_vmd::is_intel_vmd(kernel_vmd::INTEL, id), "8086:{id:04x}");
        assert!(INTEL_VMD.contains(&id), "8086:{id:04x} brought up but not named");
    }
    assert!(kernel_vmd::BUS_RESTRICTED.iter().all(|id| INTEL_VMD.contains(id)));
    assert!(INTEL_VMD.contains(&0x28c1), "the VMD the kernel leaves hidden");
}

#[test]
fn a_vmd_is_named_whatever_class_it_reports() {
    for id in INTEL_VMD {
        for (class, sub) in [(0x01, 0x04), (0x01, 0x00), (0x08, 0x80), (0x10, 0x00)] {
            let got = classify(0x8086, id, class, sub, 0);
            assert!(
                matches!(got, Some(Controller::IntelRaid)),
                "8086:{id:04x} {class:02x}/{sub:02x}"
            );
        }
    }
}
