// NONOS Operating System
// Copyright (C) 2026 NONOS Contributors
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! Clocks and yielding.

use std::sync::OnceLock;
use std::time::Instant;

#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct RtcTime {
    pub year: u16,
    pub month: u8,
    pub day: u8,
    pub hour: u8,
    pub minute: u8,
    pub second: u8,
    pub _pad: u8,
}

const ENOSYS: i64 = -38;

/*
 * There is no wall clock here, and inventing one would make a certificate
 * look valid or expired by accident. Every proof passes its own time instead,
 * and a caller of this gets the answer the kernel gives with no RTC.
 */
pub fn mk_time_rtc(_out: *mut RtcTime) -> i64 {
    ENOSYS
}

static START: OnceLock<Instant> = OnceLock::new();

pub fn mk_uptime_ms() -> i64 {
    START.get_or_init(Instant::now).elapsed().as_millis() as i64
}

pub fn mk_time_millis() -> i64 {
    mk_uptime_ms()
}

pub fn mk_yield() -> i64 {
    std::thread::yield_now();
    0
}
