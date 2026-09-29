// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root, with the modules it refers to, mirroring `src/arch/fdt/find/timer/intid.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

extern crate alloc;

pub mod arch;

pub fn entry_count(len: usize) -> usize {
    crate::arch::fdt::find::timer::intid::entry_count(len)
}

