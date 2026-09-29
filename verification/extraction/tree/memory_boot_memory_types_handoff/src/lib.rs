// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root, with the modules it refers to, mirroring `src/memory/boot_memory/types/handoff.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

extern crate alloc;

pub mod memory;

pub fn boothandoff_has_capsule(this: crate::memory::boot_memory::types::handoff::BootHandoff) -> bool {
    this.has_capsule()
}

