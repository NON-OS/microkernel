// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root mirroring `src/arch/x86_64/time/rtc/bcd.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

#[path = "../../../../../src/arch/x86_64/time/rtc/bcd.rs"]
pub mod rtc_bcd;

pub fn bcd_to_bin(bcd: u8) -> u8 {
    rtc_bcd::bcd_to_bin(bcd)
}

pub fn bin_to_bcd(bin: u8) -> u8 {
    rtc_bcd::bin_to_bcd(bin)
}

pub fn is_valid_bcd(bcd: u8) -> bool {
    rtc_bcd::is_valid_bcd(bcd)
}

