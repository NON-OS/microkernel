// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/memory/mmu/types/pte.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/memory/mmu/types/pte.rs"]
pub mod pte;

pub fn pagetableentry_empty() -> pte::PageTableEntry {
    pte::PageTableEntry::empty()
}

