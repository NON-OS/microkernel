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

//! The driver's sleep between polls. Nothing sleeps on the host: each idle
//! is counted, with the time it asked for, so a test can see a wait gave the
//! core away instead of spinning on it.

use std::cell::Cell;

thread_local! {
    static IDLES: Cell<u64> = const { Cell::new(0) };
    static IDLE_MS: Cell<u64> = const { Cell::new(0) };
}

pub fn mk_idle_ms(ms: u64) -> i64 {
    IDLES.with(|n| n.set(n.get() + 1));
    IDLE_MS.with(|t| t.set(t.get() + ms));
    0
}

/// The idles on this thread so far, and the milliseconds they asked for.
pub fn idles() -> (u64, u64) {
    (IDLES.with(Cell::get), IDLE_MS.with(Cell::get))
}
