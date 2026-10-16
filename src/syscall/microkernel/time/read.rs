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

//! The wall clock, the monotonic clock, and the NTP correction.

use core::sync::atomic::Ordering;

use super::super::errnos::ERRNO_INVAL;
use super::offset::{clamp_ok, compute_offset_ms};

pub fn sys_time_millis() -> i64 {
    if !clock_ready() {
        return -61;
    }
    let now = crate::sys::clock::unix_ms();
    now.min(i64::MAX as u64) as i64
}

// Monotonic milliseconds since boot, measured off the TSC and carrying no NTP
// offset, so a time correction can never move it backwards the way it can move
// the wall clock `sys_time_millis` returns. Zero at boot, which is what the
// `mk_uptime_ms` contract promises and what a caller reading it as an absolute
// uptime needs; callers that only take differences are unaffected.
pub fn sys_time_monotonic() -> i64 {
    crate::sys::clock::since_boot_ms().min(i64::MAX as u64) as i64
}

fn clock_ready() -> bool {
    crate::sys::clock::TSC_HZ.load(Ordering::Relaxed) != 0
        && crate::sys::clock::BOOT_UNIX_MS.load(Ordering::Relaxed) != 0
}

pub fn sys_time_adjust(correct_ms: u64) -> i64 {
    if !clamp_ok(correct_ms) {
        return ERRNO_INVAL;
    }
    let base = crate::sys::clock::base_unix_ms();
    crate::sys::clock::set_ntp_offset_ms(compute_offset_ms(correct_ms, base));
    0
}
