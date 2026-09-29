// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root, with the modules it refers to, mirroring `src/arch/x86_64/time/pit/mode.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

extern crate alloc;

pub mod arch;

pub fn mode_bits(this: crate::arch::x86_64::time::pit::mode::Mode) -> u8 {
    this.bits()
}

pub fn mode_is_periodic(this: crate::arch::x86_64::time::pit::mode::Mode) -> bool {
    this.is_periodic()
}

pub fn mode_is_oneshot(this: crate::arch::x86_64::time::pit::mode::Mode) -> bool {
    this.is_oneshot()
}

