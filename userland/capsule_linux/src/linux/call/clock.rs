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

//! The clocks a guest reads. The realtime clocks are the wall clock, the rest
//! count from boot; answering every clock with uptime put a guest in 1970 and
//! broke anything that checks a certificate's dates or a file's age.

use nonos_libc::{mk_time_millis, mk_uptime_ms};

use crate::linux::abi::errno;
use crate::linux::guest::Guest;

const CLOCK_REALTIME: u64 = 0;
const CLOCK_REALTIME_COARSE: u64 = 5;
const CLOCK_REALTIME_ALARM: u64 = 8;
const CLOCK_TAI: u64 = 11;
/// Every clock that can be named: ids past it are refused, not answered.
const CLOCK_LAST: u64 = 11;
const MS: u64 = 1_000_000;

/// Milliseconds on `clock`, or `None` for a clock Linux does not define.
pub fn now_ms(clock: u64) -> Option<u64> {
    if clock > CLOCK_LAST {
        return None;
    }
    let wall =
        matches!(clock, CLOCK_REALTIME | CLOCK_REALTIME_COARSE | CLOCK_REALTIME_ALARM | CLOCK_TAI);
    let raw = if wall { mk_time_millis() } else { mk_uptime_ms() };
    let ms = u64::try_from(raw).unwrap_or(0);
    // TAI runs ahead of UTC by the leap seconds, 37 since 2017.
    Some(if clock == CLOCK_TAI { ms.saturating_add(37_000) } else { ms })
}

pub fn clock_gettime(guest: &mut Guest, clock: u64, out: u64) -> u64 {
    let Some(ms) = now_ms(clock) else {
        return errno::fail(errno::EINVAL);
    };
    write_spec(guest, out, ms / 1000, (ms % 1000) * MS)
}

/// One millisecond: the finest step either underlying clock takes.
pub fn clock_getres(guest: &mut Guest, clock: u64, out: u64) -> u64 {
    if now_ms(clock).is_none() {
        return errno::fail(errno::EINVAL);
    }
    if out == 0 {
        return errno::ok(0);
    }
    write_spec(guest, out, 0, MS)
}

fn write_spec(guest: &mut Guest, out: u64, secs: u64, nanos: u64) -> u64 {
    let mut buf = [0u8; 16];
    buf[..8].copy_from_slice(&secs.to_le_bytes());
    buf[8..].copy_from_slice(&nanos.to_le_bytes());
    if guest.write(out, &buf) < 0 {
        return errno::fail(errno::EFAULT);
    }
    errno::ok(0)
}
