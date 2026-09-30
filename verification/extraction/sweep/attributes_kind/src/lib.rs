// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/arch/aarch64/mmu/attributes/kind.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/arch/aarch64/mmu/attributes/kind.rs"]
pub mod kind;

pub fn memorytype_attr_index(this: kind::MemoryType) -> u64 {
    this.attr_index()
}

pub fn memorytype_mair_attr(this: kind::MemoryType) -> u8 {
    this.mair_attr()
}

