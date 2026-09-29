// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root, with the modules it refers to, mirroring `src/arch/x86_64/time/pit/conversion.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

extern crate alloc;

pub mod arch;

pub fn divisor_to_frequency(divisor: u16) -> u32 {
    crate::arch::x86_64::time::pit::conversion::divisor_to_frequency(divisor)
}

pub fn divisor_to_period_ns(divisor: u16) -> u64 {
    crate::arch::x86_64::time::pit::conversion::divisor_to_period_ns(divisor)
}

pub fn frequency_error(desired_hz: u32, divisor: u16) -> i32 {
    crate::arch::x86_64::time::pit::conversion::frequency_error(desired_hz, divisor)
}

