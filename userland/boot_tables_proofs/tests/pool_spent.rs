// NONOS Operating System (AGPL-3.0-or-later)
//! 64 GiB of RAM: more gigabytes than level 2 tables. The identity map takes
//! what it needs, the direct map stops where the pool does and says where, and
//! a register window with no table left is not mapped over something else.

use boot_tables_proofs::probe::{self, KERNEL_SPACE_START};
use boot_tables_proofs::{BootInfo, MemoryRegion, MemoryType, Window};

const GIB: u64 = 1 << 30;

#[test]
fn the_direct_map_ends_where_the_pool_does() {
    probe::build(&BootInfo {
        regions: vec![MemoryRegion {
            base: GIB,
            size: 64 * GIB,
            region_type: MemoryType::Available,
        }],
        uart: Window { base: 0x0900_0000, size: 0x1000 },
        ..BootInfo::default()
    });
    // 72 tables: 64 for the identity map, the 8 left for the direct map.
    assert_eq!(probe::walk(64 * GIB).unwrap().output, 64 * GIB);
    assert_eq!(probe::direct_map_end(), 9 * GIB);
    assert_eq!(probe::walk(KERNEL_SPACE_START + 9 * GIB - 1).unwrap().level, 2);
    assert!(probe::walk(KERNEL_SPACE_START + 9 * GIB).is_none());
    assert!(probe::walk(0x0900_0000).is_none());
}
