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

//! The boot uptime clock, per test thread: it starts at zero, moves only
//! when a wait or a sleep takes time, and can be made to fail.

use std::cell::Cell;

thread_local! {
    static NOW: Cell<u64> = const { Cell::new(0) };
    static BROKEN: Cell<bool> = const { Cell::new(false) };
}

/// Milliseconds since boot, or -1 while the clock is broken.
pub fn mk_uptime_ms() -> i64 {
    if BROKEN.with(Cell::get) {
        return -1;
    }
    NOW.with(Cell::get) as i64
}

/// A sleep takes exactly the time asked.
pub fn mk_idle_ms(ms: u64) -> i64 {
    advance(ms);
    0
}

pub fn now() -> u64 {
    NOW.with(Cell::get)
}

pub fn advance(ms: u64) {
    NOW.with(|n| n.set(n.get().saturating_add(ms)));
}

/// From now on every uptime read fails (or, with `false`, reads again).
pub fn break_clock(broken: bool) {
    BROKEN.with(|b| b.set(broken));
}
