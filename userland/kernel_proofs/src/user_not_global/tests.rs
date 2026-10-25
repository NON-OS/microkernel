// NONOS Operating System (AGPL-3.0-or-later)
//! Every permission set and every entry shape: GLOBAL only on a kernel page.

use super::mmu::constants::PTE_GLOBAL as MMU_GLOBAL;
use super::mmu::types::pte::PageTableEntry;
use crate::memory::paging::constants::{PERM_GLOBAL, PERM_USER, PTE_GLOBAL};
use crate::memory::paging::types::permissions::PagePermissions;

#[test]
fn to_pte_flags_never_puts_global_on_a_user_page() {
    for bits in 0u32..1 << 13 {
        let flags = PagePermissions::from_bits(bits).to_pte_flags();
        let (user, asked) = (bits & PERM_USER != 0, bits & PERM_GLOBAL != 0);
        assert_eq!(flags & PTE_GLOBAL != 0, asked && !user, "bits {bits:#x}");
    }
}

#[test]
fn to_raw_never_puts_global_on_a_user_entry() {
    for shape in 0u32..1 << 10 {
        let bit = |i: u32| shape & (1 << i) != 0;
        let e = PageTableEntry {
            present: bit(0),
            writable: bit(1),
            user_accessible: bit(2),
            write_through: bit(3),
            cache_disabled: bit(4),
            accessed: bit(5),
            dirty: bit(6),
            huge_page: bit(7),
            global: bit(8),
            no_execute: bit(9),
            physical_address: 0x1234_5000,
        };
        let raw = e.to_raw();
        assert_eq!(raw & MMU_GLOBAL != 0, e.global && !e.user_accessible, "shape {shape:#x}");
    }
}

#[test]
fn a_kernel_page_keeps_global_when_asked() {
    assert!(PagePermissions::from_bits(PERM_GLOBAL).to_pte_flags() & PTE_GLOBAL != 0);
    let e = PageTableEntry { present: true, global: true, ..PageTableEntry::empty() };
    assert!(e.to_raw() & MMU_GLOBAL != 0);
}
