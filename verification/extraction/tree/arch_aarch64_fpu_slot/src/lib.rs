// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root, with the modules it refers to, mirroring `src/arch/aarch64/fpu/slot.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

extern crate alloc;

pub mod arch;

pub fn fpsimdslot_zeroed() -> crate::arch::aarch64::fpu::slot::FpSimdSlot {
    crate::arch::aarch64::fpu::slot::FpSimdSlot::zeroed()
}

