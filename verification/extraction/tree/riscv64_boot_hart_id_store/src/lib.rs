// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root, with the modules it refers to, mirroring `src/arch/riscv64/boot/hart_id/store.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

extern crate alloc;

pub mod arch;

pub fn store(hart_id: u64) {
    crate::arch::riscv64::boot::hart_id::store::store(hart_id)
}

