// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root, with the modules it refers to, mirroring `src/arch/riscv64/cpu/extensions/query.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

extern crate alloc;

pub mod arch;

pub fn has_vector() -> bool {
    crate::arch::riscv64::cpu::extensions::query::has_vector()
}

pub fn has_compressed() -> bool {
    crate::arch::riscv64::cpu::extensions::query::has_compressed()
}

pub fn has_atomics() -> bool {
    crate::arch::riscv64::cpu::extensions::query::has_atomics()
}

pub fn has_float() -> bool {
    crate::arch::riscv64::cpu::extensions::query::has_float()
}

pub fn has_double() -> bool {
    crate::arch::riscv64::cpu::extensions::query::has_double()
}

