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

//! `struct itimerspec` in and out: interval, then value, each seconds and
//! nanoseconds. Nanoseconds round up to the millisecond the clock keeps.

use crate::linux::abi::errno;
use crate::linux::guest::Guest;

const NSEC: u64 = 1_000_000_000;
const MS: u64 = 1_000_000;

/// (interval, value) in milliseconds, or the errno a bad one earns.
pub fn read_spec(guest: &Guest, at: u64) -> Result<(u64, u64), u64> {
    let raw = guest.read(at, 32).ok_or(errno::fail(errno::EFAULT))?;
    let w = |i: usize| u64::from_le_bytes(raw[i..i + 8].try_into().unwrap_or([0; 8]));
    let ms = |s: u64, ns: u64| match ns < NSEC && (s as i64) >= 0 {
        true => Ok(s.saturating_mul(1000).saturating_add(ns.div_ceil(MS))),
        false => Err(errno::fail(errno::EINVAL)),
    };
    Ok((ms(w(0), w(8))?, ms(w(16), w(24))?))
}

/// Write (interval, value) at `at`, if the caller gave somewhere to write.
pub fn write_spec(guest: &Guest, at: u64, interval: u64, value: u64) -> u64 {
    if at == 0 {
        return errno::ok(0);
    }
    let mut b = [0u8; 32];
    for (i, ms) in [interval, value].iter().enumerate() {
        b[i * 16..i * 16 + 8].copy_from_slice(&(ms / 1000).to_le_bytes());
        b[i * 16 + 8..i * 16 + 16].copy_from_slice(&((ms % 1000) * MS).to_le_bytes());
    }
    match guest.write(at, &b) < 32 {
        true => errno::fail(errno::EFAULT),
        false => errno::ok(0),
    }
}
