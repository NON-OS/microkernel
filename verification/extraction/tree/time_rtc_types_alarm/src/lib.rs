// NONOS Operating System (AGPL-3.0-or-later)
//! Extraction root, with the modules it refers to, mirroring `src/arch/x86_64/time/rtc/types/alarm.rs`.
//!
//! The forwarding functions below exist because a Charon entry point cannot
//! name an inherent method. They add no logic.

extern crate alloc;

pub mod arch;

pub fn rtcalarm_new(hour: u8, minute: u8, second: u8) -> crate::arch::x86_64::time::rtc::types::alarm::RtcAlarm {
    crate::arch::x86_64::time::rtc::types::alarm::RtcAlarm::new(hour, minute, second)
}

pub fn rtcalarm_every_second() -> crate::arch::x86_64::time::rtc::types::alarm::RtcAlarm {
    crate::arch::x86_64::time::rtc::types::alarm::RtcAlarm::every_second()
}

pub fn rtcalarm_every_minute() -> crate::arch::x86_64::time::rtc::types::alarm::RtcAlarm {
    crate::arch::x86_64::time::rtc::types::alarm::RtcAlarm::every_minute()
}

pub fn rtcalarm_every_hour() -> crate::arch::x86_64::time::rtc::types::alarm::RtcAlarm {
    crate::arch::x86_64::time::rtc::types::alarm::RtcAlarm::every_hour()
}

