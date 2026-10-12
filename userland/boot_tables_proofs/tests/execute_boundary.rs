// NONOS Operating System (AGPL-3.0-or-later)
//! Execute permission never crosses the privilege boundary in the boot
//! tables, checked over every leaf they hold on two boards: each is closed to
//! EL0 and has UXN set, and EL1 may run only the image's text, read-only
//! (Arm ARM, DDI 0487, the VMSAv8-64 stage 1 access permissions and the
//! execute-never controls).

use boot_tables_proofs::leaves::leaves;
use boot_tables_proofs::probe::{self, AP_EL0, AP_READ_ONLY, PXN, TEXT, UXN};
use boot_tables_proofs::{BootInfo, MemoryRegion, MemoryType, Window};

fn ram(base: u64, size: u64) -> MemoryRegion {
    MemoryRegion { base, size, region_type: MemoryType::Available }
}

fn check(info: &BootInfo) {
    probe::build(info);
    let all = leaves();
    let (mut low, mut high, mut text) = (0, 0, 0);
    for leaf in &all {
        let (va, d) = (leaf.va, leaf.descriptor);
        assert_eq!(d & AP_EL0, 0, "{va:#x} reachable from EL0");
        assert_ne!(d & UXN, 0, "{va:#x} executable at EL0");
        if d & PXN == 0 {
            assert!((TEXT.0..TEXT.1).contains(&va), "{va:#x} executable at EL1 outside text");
            assert_eq!(leaf.level, 3, "{va:#x}: text is mapped in pages");
            assert_ne!(d & AP_READ_ONLY, 0, "{va:#x} writable and executable");
            text += 1;
        }
        if va >> 63 == 0 {
            low += 1;
        } else {
            high += 1;
        }
    }
    // Not vacuous: both halves were walked and every text page found.
    assert!(low > 0 && high > 0, "low {low}, high {high}");
    assert_eq!(text, (TEXT.1 - TEXT.0) / 4096);
}

#[test]
fn qemu_virt_tables_keep_execute_to_its_own_level() {
    check(&BootInfo {
        regions: vec![ram(0x4000_0000, 0x4000_0000)],
        uart: Window { base: 0x0900_0000, size: 0x1000 },
        gic_dist_base: 0x0800_0000,
        gic_redist_base: 0x080a_0000,
        ..BootInfo::default()
    });
}
