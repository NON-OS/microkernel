// NONOS Operating System (AGPL-3.0-or-later)
//! `map_page` and `unmap_page` on QEMU virt's layout: a page goes in its level
//! 3 table or nowhere, and a page inside a 2 MiB block is refused, not written
//! where no walk reaches it nor cleared with the 511 around it.

use boot_tables_proofs::probe;
use boot_tables_proofs::{BootInfo, MemoryRegion, MemoryType, Window};

#[test]
fn pages_land_in_their_table_and_blocks_are_left_alone() {
    probe::build(&BootInfo {
        regions: vec![MemoryRegion {
            base: 0x4000_0000,
            size: 0x4000_0000,
            region_type: MemoryType::Available,
        }],
        uart: Window { base: 0x0900_0000, size: 0x1000 },
        gic_dist_base: 0x0800_0000,
        gic_redist_base: 0x080a_0000,
        ..BootInfo::default()
    });
    // A gigabyte with nothing in it yet.
    probe::map_page(0x8000_3000, 0x4123_4000);
    let leaf = probe::walk(0x8000_3000).unwrap();
    assert_eq!((leaf.output, leaf.level), (0x4123_4000, 3));
    assert!(probe::walk(0x8000_4000).is_none());
    probe::unmap_page(0x8000_3000);
    assert!(probe::walk(0x8000_3000).is_none());

    // Inside RAM's 2 MiB block: neither call changes it.
    probe::map_page(0x4100_1000, 0x7000_0000);
    probe::unmap_page(0x4100_1000);
    let block = probe::walk(0x4100_1000).unwrap();
    assert_eq!((block.output, block.level), (0x4100_0000, 2));

    // Past the gigabytes with level 3 tables, or outside level 0's first entry.
    probe::map_page(0x1_0000_0000, 0x4000_0000);
    assert!(probe::walk(0x1_0000_0000).is_none());
    probe::map_page(0x80_0000_0000, 0x4000_0000);
    assert!(probe::walk(0x80_0000_0000).is_none());

    // The redistributors' two blocks are registers.
    for at in [0x080a_0000u64, 0x082a_0000] {
        assert_eq!(probe::walk(at).unwrap().attr_index(), probe::device_attr());
    }
}
