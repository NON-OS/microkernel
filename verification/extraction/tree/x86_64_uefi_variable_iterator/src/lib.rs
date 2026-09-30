// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root, with the modules it refers to, mirroring `src/arch/x86_64/uefi/variable/iterator.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

extern crate alloc;

pub mod arch;

pub fn variableiterator_new() -> crate::arch::x86_64::uefi::variable::iterator::VariableIterator {
    crate::arch::x86_64::uefi::variable::iterator::VariableIterator::new()
}

pub fn variableiterator_is_finished(this: crate::arch::x86_64::uefi::variable::iterator::VariableIterator) -> bool {
    this.is_finished()
}

