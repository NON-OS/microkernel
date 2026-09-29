// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root, with the modules it refers to, mirroring `src/memory/layout/types/section.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

extern crate alloc;

pub mod memory;

pub fn section_new(start: u64, end: u64, rx: bool, rw: bool, nx: bool, global: bool) -> crate::memory::layout::types::section::Section {
    crate::memory::layout::types::section::Section::new(start, end, rx, rw, nx, global)
}

pub fn section_size(this: crate::memory::layout::types::section::Section) -> u64 {
    this.size()
}

pub fn section_page_count(this: crate::memory::layout::types::section::Section) -> u64 {
    this.page_count()
}

pub fn section_is_empty(this: crate::memory::layout::types::section::Section) -> bool {
    this.is_empty()
}

pub fn section_contains(this: crate::memory::layout::types::section::Section, addr: u64) -> bool {
    this.contains(addr)
}

