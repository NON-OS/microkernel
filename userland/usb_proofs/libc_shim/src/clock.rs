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

//! A clock that moves only when a test advances it or the driver idles, and
//! the total the driver idled, so a test can tell a loop that waits on the
//! clock from one that sleeps through it.

use std::cell::Cell;

thread_local! {
    static NOW_MS: Cell<i64> = const { Cell::new(0) };
    static IDLED_MS: Cell<u64> = const { Cell::new(0) };
}

pub fn mk_uptime_ms() -> i64 {
    NOW_MS.with(Cell::get)
}

/// An idle moves the clock by as long as it was asked to sleep.
pub fn mk_idle_ms(ms: u64) -> i64 {
    IDLED_MS.with(|i| i.set(i.get() + ms));
    advance_ms(ms as i64);
    0
}

pub fn advance_ms(ms: i64) {
    NOW_MS.with(|n| n.set(n.get() + ms));
}

/// How long the driver has idled since the last reset.
pub fn idled_ms() -> u64 {
    IDLED_MS.with(Cell::get)
}

pub(crate) fn reset() {
    NOW_MS.with(|n| n.set(0));
    IDLED_MS.with(|i| i.set(0));
}
