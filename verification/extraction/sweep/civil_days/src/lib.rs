// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/sys/clock/civil/days.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/sys/clock/civil/days.rs"]
pub mod days;

pub fn is_leap_year(year: u16) -> bool {
    days::is_leap_year(year)
}

pub fn days_in_month(year: u16, month: u8) -> u8 {
    days::days_in_month(year, month)
}

pub fn days_in_year(year: u16) -> u16 {
    days::days_in_year(year)
}

