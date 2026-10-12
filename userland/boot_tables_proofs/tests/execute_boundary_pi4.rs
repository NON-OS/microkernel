// NONOS Operating System (AGPL-3.0-or-later)
//! The execute boundary on a Raspberry Pi 4 with 8 GB: three RAM regions, a
//! GICv2 and the peripherals sharing a gigabyte with RAM. Its own test binary
//! because the tables are built once per process.

use boot_tables_proofs::leaves::leaves;
use boot_tables_proofs::probe::{self, AP_EL0, PXN, TEXT, UXN};
use boot_tables_proofs::{BootInfo, MemoryRegion, MemoryType, Window};

#[test]
fn pi4_tables_keep_execute_to_its_own_level() {
    let ram = |base, size| MemoryRegion { base, size, region_type: MemoryType::Available };
    probe::build(&BootInfo {
        regions: vec![
            ram(0, 0x3b40_0000),
            ram(0x4000_0000, 0xbc00_0000),
            ram(0x1_0000_0000, 0x1_0000_0000),
        ],
        uart: Window { base: 0xfe21_5040, size: 0x40 },
        gic_dist_base: 0xff84_1000,
        gic_cpu_base: 0xff84_2000,
        ..BootInfo::default()
    });
    let all = leaves();
    assert!(all.len() > 4096);
    for leaf in all {
        let (va, d) = (leaf.va, leaf.descriptor);
        assert_eq!(d & AP_EL0, 0, "{va:#x} reachable from EL0");
        assert_ne!(d & UXN, 0, "{va:#x} executable at EL0");
        assert!(d & PXN != 0 || (TEXT.0..TEXT.1).contains(&va), "{va:#x} runs at EL1");
    }
}
