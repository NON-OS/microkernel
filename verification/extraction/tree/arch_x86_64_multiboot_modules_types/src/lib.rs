// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root, with the modules it refers to, mirroring `src/arch/x86_64/multiboot/modules_types.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

extern crate alloc;

pub mod arch;

pub mod memory;

pub fn moduleinfo_size(this: crate::arch::x86_64::multiboot::modules_types::ModuleInfo) -> u64 {
    this.size()
}

