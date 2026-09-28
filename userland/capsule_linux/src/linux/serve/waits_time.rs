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

//! How long a waiting call may wait, read from each call's own form of
//! timeout. None has no limit; a timeout Linux refuses is EINVAL.

use crate::linux::abi::{errno, nr, nr_path as np};
use crate::linux::guest::Guest;

/// Milliseconds, rounded up.
pub fn span(guest: &Guest, number: u64, a: &[u64; 6]) -> Result<Option<u64>, u64> {
    match number {
        nr::POLL => Ok(int_ms(a[2])),
        np::PPOLL => spec(guest, a[2], 1_000_000_000),
        np::PSELECT6 => spec(guest, a[4], 1_000_000_000),
        nr::EPOLL_PWAIT2 => spec(guest, a[3], 1_000_000_000),
        // A timeval: seconds and microseconds.
        np::SELECT => spec(guest, a[4], 1_000_000),
        // epoll_wait and epoll_pwait.
        _ => Ok(int_ms(a[3])),
    }
}

/// An int of milliseconds; a negative one has no limit.
fn int_ms(raw: u64) -> Option<u64> {
    u64::try_from(raw as u32 as i32).ok()
}

/// A timespec or timeval whose second word counts `whole` to the second.
/// A null pointer has no limit.
fn spec(guest: &Guest, at: u64, whole: i64) -> Result<Option<u64>, u64> {
    if at == 0 {
        return Ok(None);
    }
    let Some(raw) = guest.read(at, 16) else {
        return Err(errno::fail(errno::EFAULT));
    };
    let secs = i64::from_le_bytes(raw[..8].try_into().unwrap_or([0; 8]));
    let part = i64::from_le_bytes(raw[8..16].try_into().unwrap_or([0; 8]));
    if secs < 0 || !(0..whole).contains(&part) {
        return Err(errno::fail(errno::EINVAL));
    }
    let per_ms = (whole / 1000) as u64;
    Ok(Some((secs as u64).saturating_mul(1000).saturating_add((part as u64).div_ceil(per_ms))))
}
