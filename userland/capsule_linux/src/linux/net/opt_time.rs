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

//! The options that hold a number of seconds: the keepalive times, and the
//! receive and send limits.

use crate::linux::abi::errno;

/// A keepalive time or count, 1 up to Linux's most, else EINVAL.
pub fn keep(slot: &mut u32, v: u32, most: u32) -> u64 {
    if v < 1 || v > most {
        return errno::fail(errno::EINVAL);
    }
    *slot = v;
    errno::ok(0)
}

/// SO_RCVTIMEO or SO_SNDTIMEO from a struct timeval: EDOM for microseconds
/// out of range, and a negative time is no limit, as Linux treats both.
pub fn timeo(slot: &mut (u64, u64), sec: Option<u64>, usec: Option<u64>) -> u64 {
    let (Some(sec), Some(usec)) = (sec, usec) else {
        return errno::fail(errno::EINVAL);
    };
    if usec as i64 >= 1_000_000 || (usec as i64) < 0 {
        return errno::fail(errno::EDOM);
    }
    *slot = if (sec as i64) < 0 { (0, 0) } else { (sec, usec) };
    errno::ok(0)
}
