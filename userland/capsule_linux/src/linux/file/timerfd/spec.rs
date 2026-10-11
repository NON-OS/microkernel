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

//! `struct itimerspec`: the interval, then the value, each a timespec of
//! seconds and nanoseconds. Kept here in milliseconds, rounded up.

use crate::linux::abi::errno;
use crate::linux::guest::Guest;

const LEN: usize = 32;
const NSEC: i64 = 1_000_000_000;

/// `(interval, value)`, or EFAULT or EINVAL as Linux refuses them.
pub fn read_spec(guest: &Guest, at: u64) -> Result<(u64, u64), u64> {
    let Some(raw) = guest.read(at, LEN) else {
        return Err(errno::fail(errno::EFAULT));
    };
    let word = |i: usize| i64::from_le_bytes(raw[i * 8..i * 8 + 8].try_into().unwrap_or([0; 8]));
    let mut ms = [0u64; 2];
    for (k, (secs, nanos)) in [(word(0), word(1)), (word(2), word(3))].into_iter().enumerate() {
        if secs < 0 || !(0..NSEC).contains(&nanos) {
            return Err(errno::fail(errno::EINVAL));
        }
        ms[k] =
            (secs as u64).saturating_mul(1000).saturating_add((nanos as u64).div_ceil(1_000_000));
    }
    Ok((ms[0], ms[1]))
}

/// Write `(interval, value)` in milliseconds as an itimerspec.
pub fn write_spec(guest: &mut Guest, at: u64, (every, left): (u64, u64)) -> i64 {
    let mut raw = [0u8; LEN];
    for (i, ms) in [every, left].into_iter().enumerate() {
        raw[i * 16..i * 16 + 8].copy_from_slice(&(ms / 1000).to_le_bytes());
        raw[i * 16 + 8..i * 16 + 16].copy_from_slice(&((ms % 1000) * 1_000_000).to_le_bytes());
    }
    guest.write(at, &raw)
}
