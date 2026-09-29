// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root, with the modules it refers to, mirroring `src/arch/x86_64/time/rtc/types/time.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

extern crate alloc;

pub mod arch;

pub fn rtctime_new(year: u16, month: u8, day: u8, hour: u8, minute: u8, second: u8) -> crate::arch::x86_64::time::rtc::types::time::RtcTime {
    crate::arch::x86_64::time::rtc::types::time::RtcTime::new(year, month, day, hour, minute, second)
}

pub fn rtctime_to_unix_timestamp(this: crate::arch::x86_64::time::rtc::types::time::RtcTime) -> u64 {
    this.to_unix_timestamp()
}

pub fn rtctime_from_unix_timestamp(timestamp: u64) -> crate::arch::x86_64::time::rtc::types::time::RtcTime {
    crate::arch::x86_64::time::rtc::types::time::RtcTime::from_unix_timestamp(timestamp)
}

pub fn rtctime_calculate_day_of_week(this: crate::arch::x86_64::time::rtc::types::time::RtcTime) -> u8 {
    this.calculate_day_of_week()
}

pub fn rtctime_with_day_of_week(this: crate::arch::x86_64::time::rtc::types::time::RtcTime) -> crate::arch::x86_64::time::rtc::types::time::RtcTime {
    this.with_day_of_week()
}

pub fn rtctime_is_leap_year(this: crate::arch::x86_64::time::rtc::types::time::RtcTime) -> bool {
    this.is_leap_year()
}

pub fn rtctime_day_of_year(this: crate::arch::x86_64::time::rtc::types::time::RtcTime) -> u16 {
    this.day_of_year()
}

