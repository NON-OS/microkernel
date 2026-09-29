// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root, with the modules it refers to, mirroring `src/arch/x86_64/uefi/variable/variable.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

extern crate alloc;

pub mod arch;

pub fn uefivariable_data_len(this: crate::arch::x86_64::uefi::variable::variable::UefiVariable) -> usize {
    this.data_len()
}

pub fn uefivariable_is_empty(this: crate::arch::x86_64::uefi::variable::variable::UefiVariable) -> bool {
    this.is_empty()
}

pub fn uefivariable_is_non_volatile(this: crate::arch::x86_64::uefi::variable::variable::UefiVariable) -> bool {
    this.is_non_volatile()
}

pub fn uefivariable_is_runtime_accessible(this: crate::arch::x86_64::uefi::variable::variable::UefiVariable) -> bool {
    this.is_runtime_accessible()
}

pub fn uefivariable_as_bool(this: crate::arch::x86_64::uefi::variable::variable::UefiVariable) -> bool {
    this.as_bool()
}

