// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root, with the modules it refers to, mirroring `src/memory/mmu/types/pte_decode.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

extern crate alloc;

pub mod memory;

pub fn pagetableentry_from_raw(raw: u64) -> crate::memory::mmu::types::pte::PageTableEntry {
    crate::memory::mmu::types::pte::PageTableEntry::from_raw(raw)
}

