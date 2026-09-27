// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/arch/x86_64/time/rtc/types/rate.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/arch/x86_64/time/rtc/types/rate.rs"]
pub mod rate;

pub fn periodicrate_value(this: rate::PeriodicRate) -> u8 {
    this.value()
}

pub fn periodicrate_frequency_hz(this: rate::PeriodicRate) -> u32 {
    this.frequency_hz()
}

pub fn periodicrate_period_us(this: rate::PeriodicRate) -> u32 {
    this.period_us()
}

