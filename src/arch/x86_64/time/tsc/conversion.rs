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

use super::state::CALIBRATION;

/*
 * x * mul / div in 128 bits, saturating at u64::MAX. In 64 bits these
 * products overflow: ns * freq after about 6 s at 3 GHz, ticks * 1_000_000
 * after about 1.7 h of uptime. The kernel is built with overflow checks, so
 * an overflow is a panic, not a wrong time.
 */
#[inline]
fn scale(x: u64, mul: u64, div: u64) -> u64 {
    if div == 0 {
        return 0;
    }
    u64::try_from(x as u128 * mul as u128 / div as u128).unwrap_or(u64::MAX)
}

#[inline]
fn freq() -> u64 {
    CALIBRATION.read().frequency_hz
}

#[inline]
pub fn ticks_to_ns(ticks: u64) -> u64 {
    scale(ticks, 1_000_000_000, freq())
}

#[inline]
pub fn ticks_to_us(ticks: u64) -> u64 {
    scale(ticks, 1_000_000, freq())
}

#[inline]
pub fn ticks_to_ms(ticks: u64) -> u64 {
    scale(ticks, 1_000, freq())
}

#[inline]
pub fn ns_to_ticks(ns: u64) -> u64 {
    let f = freq();
    if f == 0 {
        return 0;
    }
    scale(ns, f, 1_000_000_000)
}

#[inline]
pub fn us_to_ticks(us: u64) -> u64 {
    let f = freq();
    if f == 0 {
        return 0;
    }
    scale(us, f, 1_000_000)
}

#[inline]
pub fn ms_to_ticks(ms: u64) -> u64 {
    let f = freq();
    if f == 0 {
        return 0;
    }
    scale(ms, f, 1_000)
}

pub fn tsc_to_ns(tsc_ticks: u64, tsc_freq: u64) -> u64 {
    scale(tsc_ticks, 1_000_000_000, tsc_freq)
}

pub fn ns_to_tsc(nanoseconds: u64, tsc_freq: u64) -> u64 {
    scale(nanoseconds, tsc_freq, 1_000_000_000)
}
