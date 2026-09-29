// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root, with the modules it refers to, mirroring `src/memory/heap/types/header.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

extern crate alloc;

pub mod memory;

pub fn allocationheader_new(size: usize, timestamp: u64) -> crate::memory::heap::types::header::AllocationHeader {
    crate::memory::heap::types::header::AllocationHeader::new(size, timestamp)
}

pub fn allocationheader_is_valid(this: crate::memory::heap::types::header::AllocationHeader) -> bool {
    this.is_valid()
}

