// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root, with the modules it refers to, mirroring `src/memory/page_info/types/flags.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

extern crate alloc;

pub mod memory;

pub fn pageflags_from_bits(bits: u32) -> crate::memory::page_info::types::flags::PageFlags {
    crate::memory::page_info::types::flags::PageFlags::from_bits(bits)
}

pub fn pageflags_bits(this: crate::memory::page_info::types::flags::PageFlags) -> u32 {
    this.bits()
}

pub fn pageflags_contains(this: crate::memory::page_info::types::flags::PageFlags, other: crate::memory::page_info::types::flags::PageFlags) -> bool {
    this.contains(other)
}

pub fn pageflags_union(this: crate::memory::page_info::types::flags::PageFlags, other: crate::memory::page_info::types::flags::PageFlags) -> crate::memory::page_info::types::flags::PageFlags {
    this.union(other)
}

pub fn pageflags_intersection(this: crate::memory::page_info::types::flags::PageFlags, other: crate::memory::page_info::types::flags::PageFlags) -> crate::memory::page_info::types::flags::PageFlags {
    this.intersection(other)
}

pub fn pageflags_difference(this: crate::memory::page_info::types::flags::PageFlags, other: crate::memory::page_info::types::flags::PageFlags) -> crate::memory::page_info::types::flags::PageFlags {
    this.difference(other)
}

pub fn pageflags_is_empty(this: crate::memory::page_info::types::flags::PageFlags) -> bool {
    this.is_empty()
}

