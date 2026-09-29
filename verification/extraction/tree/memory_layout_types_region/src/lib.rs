// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root, with the modules it refers to, mirroring `src/memory/layout/types/region.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

extern crate alloc;

pub mod memory;

pub fn region_len(this: crate::memory::layout::types::region::Region) -> u64 {
    this.len()
}

pub fn region_is_empty(this: crate::memory::layout::types::region::Region) -> bool {
    this.is_empty()
}

pub fn region_is_usable(this: crate::memory::layout::types::region::Region) -> bool {
    this.is_usable()
}

pub fn region_start_addr(this: crate::memory::layout::types::region::Region) -> u64 {
    this.start_addr()
}

pub fn region_end_addr(this: crate::memory::layout::types::region::Region) -> u64 {
    this.end_addr()
}

pub fn region_page_count(this: crate::memory::layout::types::region::Region) -> u64 {
    this.page_count()
}

pub fn region_contains(this: crate::memory::layout::types::region::Region, addr: u64) -> bool {
    this.contains(addr)
}

