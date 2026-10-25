// NONOS Operating System (AGPL-3.0-or-later)
//! Neither aarch64 encoder makes an EL0 page global, for every request it can
//! be given (P-06): a global entry is shared by every ASID, so it outlives the
//! switch to the next process, which could read it. A kernel mapping keeps
//! the global it asked for. The manager's `leaf` is checked over every
//! combination of the flags it reads, `PageAttributes` over all its fields.

use crate::arch::aarch64::mmu::attributes::{MemoryType, PageAttributes, PTE_NG};
use crate::descriptor::aarch64 as d;
use crate::descriptor::aarch64::bits::NOT_GLOBAL;
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
fn the_managers_leaf_never_makes_an_el0_page_global() {
    for combo in 0..1u32 << FLAGS.len() {
        let f = request(combo);
        let entry = d::leaf(0x1234_5000, f);
        let (user, global) = (f & flags::USER != 0, f & flags::GLOBAL != 0);
        assert_eq!(entry & NOT_GLOBAL == 0, global && !user, "{f:#x}");
    }
}

#[test]
fn page_attributes_never_make_an_el0_page_global() {
    for memory_type in [MemoryType::DeviceNGnRnE, MemoryType::NormalNC, MemoryType::NormalWB] {
        for fields in 0..1u32 << 8 {
            let bit = |n: u32| fields >> n & 1 != 0;
            let mut a =
                PageAttributes::new(memory_type, bit(0), bit(1), bit(2), bit(3), bit(4), bit(5));
            a.contiguous = bit(6);
            a.guarded = bit(7);
            let bits = a.to_descriptor_bits();
            assert_eq!(bits & PTE_NG == 0, a.global && !a.user, "{a:?}");
        }
    }
}
