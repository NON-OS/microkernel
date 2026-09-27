// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/arch/x86_64/uefi/tables/memory_desc.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/arch/x86_64/uefi/tables/memory_desc.rs"]
pub mod memory_desc;

pub fn memorydescriptor_size_bytes(this: memory_desc::MemoryDescriptor) -> u64 {
    this.size_bytes()
}

pub fn memorydescriptor_end_address(this: memory_desc::MemoryDescriptor) -> u64 {
    this.end_address()
}

pub fn memorydescriptor_is_runtime(this: memory_desc::MemoryDescriptor) -> bool {
    this.is_runtime()
}

pub fn memorydescriptor_is_usable(this: memory_desc::MemoryDescriptor) -> bool {
    this.is_usable()
}

