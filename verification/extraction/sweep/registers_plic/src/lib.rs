// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/arch/riscv64/plic/registers/plic.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/arch/riscv64/plic/registers/plic.rs"]
pub mod plic;

pub fn plic_new(base: u64) -> plic::Plic {
    plic::Plic::new(base)
}

pub fn plic_base(this: plic::Plic) -> u64 {
    this.base()
}

