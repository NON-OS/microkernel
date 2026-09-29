// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root, with the modules it refers to, mirroring `src/arch/riscv64/cpu/caps/query.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

extern crate alloc;

pub mod arch;

pub fn is_configured() -> bool {
    crate::arch::riscv64::cpu::caps::query::is_configured()
}

pub fn has_f() -> bool {
    crate::arch::riscv64::cpu::caps::query::has_f()
}

pub fn has_d() -> bool {
    crate::arch::riscv64::cpu::caps::query::has_d()
}

pub fn has_v() -> bool {
    crate::arch::riscv64::cpu::caps::query::has_v()
}

pub fn has_a() -> bool {
    crate::arch::riscv64::cpu::caps::query::has_a()
}

pub fn has_c() -> bool {
    crate::arch::riscv64::cpu::caps::query::has_c()
}

