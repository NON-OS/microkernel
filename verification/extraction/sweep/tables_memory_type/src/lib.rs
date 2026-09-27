// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/arch/x86_64/uefi/tables/memory_type.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/arch/x86_64/uefi/tables/memory_type.rs"]
pub mod memory_type;

pub fn memorytype_is_usable(this: memory_type::MemoryType) -> bool {
    this.is_usable()
}

pub fn memorytype_is_reserved(this: memory_type::MemoryType) -> bool {
    this.is_reserved()
}

