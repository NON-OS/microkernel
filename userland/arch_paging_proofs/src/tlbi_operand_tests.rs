// NONOS Operating System (AGPL-3.0-or-later)
//! The `TLBI VAAE1IS` operand names the page asked for and nothing else: its
//! TTL hint (bits 47:44) and its RES0 bits (63:48) are zero for every address,
//! and bits 43:0 are VA[55:12] (Arm ARM, DDI 0487, the TLBI VAAE1IS operand).
//! A non-zero TTL on a FEAT_TTL CPU names a granule and level the kernel's
//! 4K tables do not use, and an invalidate the hint does not describe need not
//! remove anything, so a wrong hint is a stale translation that no QEMU run
//! shows.

use crate::arch::aarch64::mmu::tlbi_operand::{vaae1is, VA_FIELD};

const TTL: u64 = 0xF << 44;
const RES0: u64 = 0xFFFF << 48;

/// Addresses from both halves of the 48-bit space and the edges between.
const ADDRESSES: [u64; 12] = [
    0,
    0x1000,
    0x0000_7FFF_FFFF_F000,
    0x0000_8000_0000_0000,
    0x4008_0000,
    0x0000_0040_2000_0000,
    0xFFFF_8000_0000_0000,
    0xFFFF_8000_1234_5000,
    0xFFFF_8000_4008_0FFF,
    0xFFFF_FFFF_FFFF_F000,
    0xFFFF_FFFF_FFFF_FFFF,
    0xDEAD_BEEF_CAFE_F00D,
];

#[test]
fn ttl_and_res0_are_zero_for_every_address() {
    for &va in &ADDRESSES {
        let op = vaae1is(va);
        assert_eq!(op & TTL, 0, "TTL hint set for {va:#x}: {op:#x}");
        assert_eq!(op & RES0, 0, "RES0 bits set for {va:#x}: {op:#x}");
    }
    // A sweep over the top bits, where kernel addresses differ from user ones.
    for top in 0..=0xFFFFu64 {
        let va = top << 48 | 0x0000_8765_4321_0000;
        assert_eq!(vaae1is(va) & !VA_FIELD, 0, "{va:#x}");
    }
}

#[test]
fn the_field_is_va_55_to_12() {
    for &va in &ADDRESSES {
        assert_eq!(vaae1is(va) << 12, va & 0x00FF_FFFF_FFFF_F000, "{va:#x}");
    }
}

#[test]
fn a_kernel_address_is_the_case_that_needs_the_mask() {
    // Unmasked, the kernel direct map's first page puts 0b1111 into TTL:
    // 64K granule, level 3, which is what the masking exists to stop.
    let va = 0xFFFF_8000_0000_0000u64;
    assert_eq!((va >> 12) & TTL, TTL);
    assert_eq!(vaae1is(va), 0x0FF8_0000_0000);
    assert_eq!(vaae1is(0xFFFF_8000_1234_5000), 0x0FF8_0001_2345);
}

#[test]
fn the_offset_within_a_page_does_not_change_the_operand() {
    for &va in &ADDRESSES {
        let page = va & !0xFFF;
        assert_eq!(vaae1is(va), vaae1is(page), "{va:#x}");
    }
}

#[test]
fn distinct_pages_in_one_half_get_distinct_operands() {
    // Within the TTBR1 half (and within TTBR0) VA[55:12] is the whole page
    // number, so two different pages never share an invalidate.
    let base = 0xFFFF_8000_0000_0000u64;
    let mut seen = std::collections::BTreeSet::new();
    for i in 0..4096u64 {
        assert!(seen.insert(vaae1is(base + (i << 12) * 0x101)));
    }
}
