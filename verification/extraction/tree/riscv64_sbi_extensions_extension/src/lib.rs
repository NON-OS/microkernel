// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root, with the modules it refers to, mirroring `src/arch/riscv64/sbi/extensions/extension.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

extern crate alloc;

pub mod arch;

pub fn extension_eid(this: crate::arch::riscv64::sbi::extensions::extension::Extension) -> usize {
    this.eid()
}

