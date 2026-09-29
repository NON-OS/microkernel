// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root, with the modules it refers to, mirroring `src/sys/timer/tsc/convert.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

extern crate alloc;

pub mod sys;

pub fn tsc_frequency() -> u64 {
    crate::sys::timer::tsc::convert::tsc_frequency()
}

pub fn ticks_to_ns(ticks: u64) -> u64 {
    crate::sys::timer::tsc::convert::ticks_to_ns(ticks)
}

pub fn ticks_to_us(ticks: u64) -> u64 {
    crate::sys::timer::tsc::convert::ticks_to_us(ticks)
}

pub fn ticks_to_ms(ticks: u64) -> u64 {
    crate::sys::timer::tsc::convert::ticks_to_ms(ticks)
}

pub fn us_to_ticks(us: u64) -> u64 {
    crate::sys::timer::tsc::convert::us_to_ticks(us)
}

pub fn ms_to_ticks(ms: u64) -> u64 {
    crate::sys::timer::tsc::convert::ms_to_ticks(ms)
}

