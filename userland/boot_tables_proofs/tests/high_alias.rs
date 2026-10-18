// NONOS Operating System (AGPL-3.0-or-later)
//! The image and the boot devices at their high alias, which the kernel runs
//! from once it moves to TTBR1: every image page there is the same physical
//! page with the same descriptor bits as one to one, so nothing gains or
//! loses a permission in the move, and each boot device is Device memory.

use boot_tables_proofs::probe::{self, IMAGE, KERNEL_SPACE_START};
use boot_tables_proofs::{BootInfo, MemoryRegion, MemoryType, Window};

/// Everything a descriptor says but its output address.
const NOT_ADDRESS: u64 = !0x0000_FFFF_FFFF_F000;

#[test]
fn qemu_virt_image_and_devices_alias_exactly() {
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
    assert!(probe::high_mapped());
    for page in (IMAGE.0..IMAGE.1).step_by(4096) {
        let low = probe::walk(page).unwrap();
        let high = probe::walk(KERNEL_SPACE_START + page).unwrap();
        assert_eq!((high.output, high.level), (page, 3), "{page:#x}");
        assert_eq!(high.descriptor & NOT_ADDRESS, low.descriptor & NOT_ADDRESS, "{page:#x}");
        assert_eq!(high.executable(), probe::is_text(page), "{page:#x}");
    }
    for register in [0x0900_0000u64, 0x0800_0000, 0x080a_0000, 0x080a_0000 + (3 << 20)] {
        let high = probe::walk(KERNEL_SPACE_START + register).unwrap();
        assert_eq!(high.output, register & !((2 << 20) - 1));
        assert_eq!(high.attr_index(), probe::device_attr());
        assert!(!high.executable());
    }
}
