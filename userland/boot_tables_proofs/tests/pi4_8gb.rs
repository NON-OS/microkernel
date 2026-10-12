// NONOS Operating System (AGPL-3.0-or-later)
//! A Raspberry Pi 4 with 8 GB: RAM in three regions over eight gigabytes, the
//! top of the second sharing a gigabyte with the peripherals, and a GICv2,
//! so no redistributor window.

use boot_tables_proofs::probe::{self, KERNEL_SPACE_START};
use boot_tables_proofs::{BootInfo, MemoryRegion, MemoryType, Window};

const MIB2: u64 = 2 << 20;
const RAM: [(u64, u64); 3] =
    [(0, 0x3b40_0000), (0x4000_0000, 0xbc00_0000), (0x1_0000_0000, 0x1_0000_0000)];

fn pi4() -> BootInfo {
    let region = |(base, size)| MemoryRegion { base, size, region_type: MemoryType::Available };
    BootInfo {
        regions: RAM.iter().copied().map(region).collect(),
        uart: Window { base: 0xfe21_5040, size: 0x40 },
        gic_dist_base: 0xff84_1000,
        gic_cpu_base: 0xff84_2000,
        ..BootInfo::default()
    }
}

#[test]
fn every_block_of_every_region_and_the_registers_beside_them() {
    probe::build(&pi4());
    let image = probe::IMAGE.0 & !(MIB2 - 1)..probe::IMAGE.1;
    for (base, size) in RAM {
        for phys in (base..base + size).step_by(MIB2 as usize) {
            // Identity: the image's blocks are pages, the rest one block.
            let leaf = probe::walk(phys).unwrap_or_else(|| panic!("{phys:#x} unmapped"));
            assert_eq!(leaf.output, phys);
            assert_eq!(leaf.level, if image.contains(&phys) { 3 } else { 2 }, "{phys:#x}");
            assert_eq!(leaf.attr_index(), probe::ram_attr(), "{phys:#x} is not RAM");
            // Direct: every block, never executable.
            let direct = probe::walk(KERNEL_SPACE_START + phys).unwrap();
            assert_eq!((direct.output, direct.level), (phys, 2));
            assert!(!direct.executable());
        }
        assert!(probe::walk(base + size).is_none_or(|l| l.attr_index() == probe::device_attr()));
    }
    for register in [0xfe21_5040u64, 0xff84_1000, 0xff84_2000] {
        let leaf = probe::walk(register).unwrap();
        assert_eq!((leaf.output, leaf.level), (register & !(MIB2 - 1), 2));
        assert_eq!(leaf.attr_index(), probe::device_attr());
        assert!(!leaf.executable());
    }
    // Text runs, read-only data does not.
    assert!(probe::walk(probe::TEXT.0).unwrap().executable());
    assert!(!probe::walk(probe::TEXT.1).unwrap().executable());
    assert_eq!(probe::direct_map_end(), u64::MAX);
    assert!(boot_tables_proofs::sys::serial::LINES.lock().unwrap().is_empty());
}
