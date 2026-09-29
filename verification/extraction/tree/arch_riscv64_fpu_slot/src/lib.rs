// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root, with the modules it refers to, mirroring `src/arch/riscv64/fpu/slot.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

extern crate alloc;

pub mod arch;

pub fn fpslot_zeroed() -> crate::arch::riscv64::fpu::slot::FpSlot {
    crate::arch::riscv64::fpu::slot::FpSlot::zeroed()
}

