// NONOS Operating System (AGPL-3.0-or-later)
//! Which PCI functions the driver takes for an AHCI HBA. The ids are real
//! ones: QEMU's ICH9, Intel Gemini Lake, Intel RST in RAID mode, AMD FCH,
//! and the Intel VMDs that look like RAID controllers and are not HBAs.

use crate::discover::rule::{abar_usable, is_ahci_function, INTEL_VMD_DEVICE_IDS, MIN_ABAR_BYTES};
use crate::kernel_vmd;

#[test]
fn the_named_sata_controllers_are_taken() {
    let named = [
        (0x01, 0x06, 0x8086, 0x2922, "QEMU ICH9"),
        (0x01, 0x06, 0x8086, 0x31e3, "Gemini Lake (Celeron N4120)"),
        (0x01, 0x06, 0x8086, 0xa352, "Cannon Lake PCH"),
        (0x01, 0x06, 0x1022, 0x7901, "AMD FCH"),
        (0x01, 0x06, 0x1022, 0x7801, "AMD Hudson-2"),
        (0x01, 0x06, 0x1022, 0x43b7, "AMD Promontory"),
        (0x01, 0x04, 0x8086, 0x282a, "Intel RST RAID On, mobile"),
        (0x01, 0x04, 0x8086, 0x2822, "Intel RST RAID On, desktop"),
    ];
    for (class, sub, vendor, device, what) in named {
        assert!(is_ahci_function(class, sub, vendor, device), "{what}");
    }
}

#[test]
fn what_is_not_an_hba_is_left_alone() {
    let named = [
        (0x01, 0x04, 0x8086, 0x9a0b, "Tiger Lake VMD"),
        (0x01, 0x04, 0x8086, 0x467f, "Alder Lake VMD"),
        (0x01, 0x06, 0x8086, 0x467f, "a VMD id under the SATA subclass"),
        (0x01, 0x04, 0x1022, 0x7916, "AMD RAIDXpert, not Intel"),
        (0x01, 0x08, 0x8086, 0xf1a8, "an NVMe controller"),
        (0x01, 0x01, 0x8086, 0x7010, "PIIX IDE"),
        (0x02, 0x06, 0x8086, 0x31e3, "a SATA id under another class"),
    ];
    for (class, sub, vendor, device, what) in named {
        assert!(!is_ahci_function(class, sub, vendor, device), "{what}");
    }
}

#[test]
fn every_vmd_id_is_refused_and_matches_the_kernel_list() {
    assert_eq!(INTEL_VMD_DEVICE_IDS, kernel_vmd::INTEL_VMD_DEVICE_IDS);
    for id in INTEL_VMD_DEVICE_IDS {
        assert!(kernel_vmd::is_intel_vmd(0x8086, id));
        assert!(!is_ahci_function(0x01, 0x04, 0x8086, id), "{id:#x}");
        assert!(!is_ahci_function(0x01, 0x06, 0x8086, id), "{id:#x}");
        // Another vendor's function with that id is no VMD.
        assert!(!kernel_vmd::is_intel_vmd(0x1022, id));
    }
}

#[test]
fn an_abar_holding_one_port_is_enough() {
    assert_eq!(MIN_ABAR_BYTES, 0x180);
    assert!(!abar_usable(0x100), "the global registers alone");
    assert!(!abar_usable(0x17f));
    assert!(abar_usable(0x180));
    assert!(abar_usable(0x400), "AMD FCH 1 KiB");
    assert!(abar_usable(0x800), "Intel PCH 2 KiB");
    assert!(abar_usable(0x1000), "QEMU 4 KiB");
}
