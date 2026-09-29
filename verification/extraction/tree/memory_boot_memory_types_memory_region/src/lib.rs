// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root, with the modules it refers to, mirroring `src/memory/boot_memory/types/memory_region.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

extern crate alloc;

pub mod memory;

pub fn memoryregion_size(this: crate::memory::boot_memory::types::memory_region::MemoryRegion) -> u64 {
    this.size()
}

pub fn memoryregion_page_count(this: crate::memory::boot_memory::types::memory_region::MemoryRegion) -> u64 {
    this.page_count()
}

pub fn memoryregion_is_available(this: crate::memory::boot_memory::types::memory_region::MemoryRegion) -> bool {
    this.is_available()
}

pub fn memoryregion_is_empty(this: crate::memory::boot_memory::types::memory_region::MemoryRegion) -> bool {
    this.is_empty()
}

pub fn memoryregion_has_flag(this: crate::memory::boot_memory::types::memory_region::MemoryRegion, flag: u32) -> bool {
    this.has_flag(flag)
}

