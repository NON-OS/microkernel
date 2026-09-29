// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root, with the modules it refers to, mirroring `src/arch/x86_64/vga/ops/lock.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

extern crate alloc;

pub mod arch;

pub fn acquire_lock() -> bool {
    crate::arch::x86_64::vga::ops::lock::acquire_lock()
}

pub fn release_lock() {
    crate::arch::x86_64::vga::ops::lock::release_lock()
}

