// NONOS Operating System (AGPL-3.0-or-later)
//! Neither aarch64 encoder lets execute permission cross the privilege
//! boundary, for every request it can be given: a kernel mapping always
//! carries UXN and a user mapping always carries PXN (Arm ARM, DDI 0487, the
//! stage 1 execute-never controls). The shared manager's `leaf` is checked
//! over every combination of the flags it reads, the boot tables'
//! `PageAttributes` over every combination of its fields.

use crate::arch::aarch64::mmu::attributes::{
    MemoryType, PageAttributes, PTE_AP_RW_ALL as AP_EL0, PTE_GP as GP, PTE_PXN as PXN,
    PTE_UXN as UXN,
};
use crate::descriptor::aarch64 as d;
use crate::descriptor::flags;

const FLAGS: [u64; 11] = [
    flags::PRESENT,
    flags::WRITABLE,
    flags::USER,
    flags::WRITE_THROUGH,
    flags::NO_CACHE,
    flags::ACCESSED,
    flags::DIRTY,
    flags::HUGE,
    flags::GLOBAL,
    flags::DEVICE,
    flags::NO_EXECUTE,
];

fn request(combo: u32) -> u64 {
    FLAGS.iter().enumerate().filter(|(bit, _)| combo >> bit & 1 != 0).map(|(_, f)| *f).sum()
}

#[test]
fn the_managers_leaf_never_lets_execute_cross_levels() {
    for combo in 0..1u32 << FLAGS.len() {
        let f = request(combo);
        let entry = d::leaf(0x1234_5000, f);
        let (user, exec) = (f & flags::USER != 0, f & flags::NO_EXECUTE == 0);
        assert_eq!(entry & AP_EL0 != 0, user, "{f:#x}");
        assert_eq!(entry & PXN == 0, !user && exec, "{f:#x}: EL1 execute");
        assert_eq!(entry & UXN == 0, user && exec, "{f:#x}: EL0 execute");
    }
}

#[test]
fn page_attributes_never_let_execute_cross_levels() {
    let types = [MemoryType::DeviceNGnRnE, MemoryType::NormalNC, MemoryType::NormalWB];
    for memory_type in types {
        for fields in 0..1u32 << 8 {
            let bit = |n: u32| fields >> n & 1 != 0;
            let mut a =
                PageAttributes::new(memory_type, bit(0), bit(1), bit(2), bit(3), bit(4), bit(5));
            a.contiguous = bit(6);
            a.guarded = bit(7);
            let bits = a.to_descriptor_bits();
            assert_eq!(bits & AP_EL0 != 0, a.user, "{a:?}");
            assert_eq!(bits & PXN == 0, !a.user && a.execute, "{a:?}: EL1 execute");
            assert_eq!(bits & UXN == 0, a.user && a.execute, "{a:?}: EL0 execute");
            let el1_code = a.execute && !a.user;
            assert_eq!(bits & GP != 0, a.guarded && el1_code, "{a:?}: guarded");
        }
    }
}

#[test]
fn the_named_constructors_hold_to_it() {
    let kernel =
        [PageAttributes::kernel_code(), PageAttributes::kernel_data(), PageAttributes::device()];
    for a in kernel {
        assert_ne!(a.to_descriptor_bits() & UXN, 0, "{a:?}");
    }
    for a in [PageAttributes::user_code(), PageAttributes::user_data()] {
        assert_ne!(a.to_descriptor_bits() & PXN, 0, "{a:?}");
    }
    assert_eq!(PageAttributes::kernel_code().to_descriptor_bits() & PXN, 0);
    assert_eq!(PageAttributes::user_code().to_descriptor_bits() & UXN, 0);
}
