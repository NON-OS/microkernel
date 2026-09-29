// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/memory/mmu/types/protection.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/memory/mmu/types/protection.rs"]
pub mod protection;

pub fn protectionflags_new() -> protection::ProtectionFlags {
    protection::ProtectionFlags::new()
}

pub fn protectionflags_is_fully_protected(this: protection::ProtectionFlags) -> bool {
    this.is_fully_protected()
}

