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

//! The wall clock, and deadlines on the uptime (libc's own deadline file).

pub use crate::calls::mk_uptime_ms;

#[path = "../../../libc/src/time/deadline.rs"]
mod deadline;
pub use deadline::Deadline;

#[repr(C)]
#[derive(Clone, Copy, Default)]
pub struct RtcTime {
    pub year: u16,
    pub month: u8,
    pub day: u8,
    pub hour: u8,
    pub minute: u8,
    pub second: u8,
    pub _pad: u8,
}

/// 24 October 2026, 10:00:00, as mk_time_millis says.
pub fn mk_time_rtc(out: *mut RtcTime) -> i64 {
    let t = RtcTime { year: 2026, month: 10, day: 24, hour: 10, minute: 0, second: 0, _pad: 0 };
    // SAFETY: the caller hands a writable RtcTime, as the real call requires.
    unsafe { crate::user_mem::put(out as usize, t) };
    0
}
