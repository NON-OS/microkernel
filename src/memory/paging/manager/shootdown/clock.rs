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

//! The wait's clock: the counter, and a deadline in its ticks.

/// `ms` as counter ticks, or `fallback` when the counter is not calibrated.
pub(super) fn budget(ms: u64, fallback: u64) -> u64 {
    match crate::sys::timer::tsc::tsc_frequency() / 1000 * ms {
        0 => fallback,
        ticks => ticks,
    }
}

#[inline]
pub(super) fn read_tsc() -> u64 {
    // SAFETY: eK@nonos.systems — rdtsc has no side effects and is
    // unconditionally available on every x86_64 CPU NØNOS supports.
    crate::arch::read_time_counter()
}
