// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root, with the modules it refers to, mirroring `src/memory/paging/types/page_size.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

extern crate alloc;

pub mod arch;

pub mod memory;

pub fn pagesize_bytes(this: crate::memory::paging::types::page_size::PageSize) -> usize {
    this.bytes()
}

pub fn pagesize_align_mask(this: crate::memory::paging::types::page_size::PageSize) -> u64 {
    this.align_mask()
}

pub fn pagesize_is_aligned(this: crate::memory::paging::types::page_size::PageSize, addr: u64) -> bool {
    this.is_aligned(addr)
}

