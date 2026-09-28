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

/* utimensat and utimes: the times the family sets on its own files. */

use crate::linux::abi::errno;
use crate::linux::guest::Guest;

use super::super::flags::AT_FDCWD;
use super::stamp::stamp;

const UTIME_NOW: u64 = (1 << 30) - 1;

const UTIME_OMIT: u64 = (1 << 30) - 2;

/*
 * Times are not kept, so only a call that changes neither is answered.
 * utimensat and futimens (a null path names `dirfd` itself). Each time is
 * a timespec, UTIME_NOW or UTIME_OMIT; a null array means now for both.
 */
pub fn utimensat(guest: &Guest, dirfd: u64, path: u64, times: u64, flags: u64) -> u64 {
    let spec = |at: u64| -> Result<Option<u64>, i64> {
        let raw = guest.read(at, 16).ok_or(errno::EFAULT)?;
        let secs = u64::from_le_bytes(raw[..8].try_into().unwrap_or([0; 8]));
        let nanos = u64::from_le_bytes(raw[8..].try_into().unwrap_or([0; 8]));
        match nanos {
            UTIME_OMIT => Ok(None),
            UTIME_NOW => Ok(Some(super::super::meta::now_ms())),
            n if n >= 1_000_000_000 => Err(errno::EINVAL),
            n => Ok(Some(secs.saturating_mul(1000) + n / 1_000_000)),
        }
    };
    let pair = match times {
        0 => Ok((Some(super::super::meta::now_ms()), Some(super::super::meta::now_ms()))),
        t => spec(t).and_then(|a| spec(t + 16).map(|m| (a, m))),
    };
    match pair {
        Ok((a, m)) => stamp(guest, dirfd, path, flags, a, m),
        Err(e) => errno::fail(e),
    }
}

/* utimes and utime: seconds and microseconds, or whole seconds. */
pub fn utimes(guest: &Guest, path: u64, times: u64, micros: bool) -> u64 {
    if times == 0 {
        return utimensat(guest, AT_FDCWD, path, 0, 0);
    }
    let width = if micros { 16 } else { 8 };
    let Some(raw) = guest.read(times, width * 2) else {
        return errno::fail(errno::EFAULT);
    };
    let word = |at: usize| u64::from_le_bytes(raw[at..at + 8].try_into().unwrap_or([0; 8]));
    let ms = |at: usize| word(at) * 1000 + if micros { word(at + 8) / 1000 } else { 0 };
    stamp(guest, AT_FDCWD, path, 0, Some(ms(0)), Some(ms(width)))
}
