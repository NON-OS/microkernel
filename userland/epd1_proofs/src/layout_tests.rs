// NONOS Operating System (AGPL-3.0-or-later)
//! The trampoline's slot belongs to it alone: above every user address,
//! inside TTBR0's 48 bits, at the address the vectors test for, and holding
//! none of the kernel's own regions in TTBR1.

use crate::arch::aarch64::mmu::geometry::{BASE, SLOT};
use crate::regions::{layout_addresses, sized_regions};
use crate::source::define;

const USER_END: u64 = 0x0000_8000_0000_0000;

fn slot_of(va: u64) -> usize {
    (va >> 39 & 0x1FF) as usize
}

#[test]
fn the_slot_is_above_user_space_and_inside_ttbr0() {
    const { assert!(BASE >= USER_END && BASE < 1 << 48) };
    assert_eq!(slot_of(BASE), SLOT);
    assert_eq!(BASE & ((1 << 39) - 1), 0, "the first page of its slot");
}

/// The exit test is `eor x30, x30, #EPD1_TRAMPOLINE`, and `eor` takes a
/// logical immediate: one run of set bits (Arm ARM, DDI 0487, C6).
#[test]
fn the_vectors_test_the_same_address_the_tables_map() {
    assert_eq!(define("EPD1_TRAMPOLINE"), BASE);
    assert_eq!(define("TCR_EPD1"), 1 << 23);
    let shifted = BASE >> BASE.trailing_zeros();
    assert_eq!(shifted & (shifted + 1), 0);
}

#[test]
fn no_kernel_region_shares_the_slot() {
    let addresses = layout_addresses();
    assert!(addresses.len() >= 10, "the layout's regions were not found");
    for va in addresses {
        assert_ne!(slot_of(va), SLOT, "{va:#x} lies in the trampoline's slot");
    }
}

#[test]
fn no_kernel_region_reaches_into_the_slot() {
    let regions = sized_regions();
    assert!(!regions.is_empty(), "no sized region was found");
    let (lo, hi) = (0xFFFF_0000_0000_0000 | BASE, (0xFFFF_0000_0000_0000 | BASE) + (1 << 39));
    for (start, end) in regions {
        assert!(end <= lo || start >= hi, "{start:#x}..{end:#x} meets the slot");
    }
}
