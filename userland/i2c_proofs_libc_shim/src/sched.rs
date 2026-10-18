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

//! Yielding. A yield is counted rather than taken, so a test can see how a
//! driver waited. A test can also make the scheduler busy: every yield then
//! hands a whole slice to other work, and the clock (time.rs) moves on by it.

use std::cell::Cell;

thread_local! {
    static YIELDS: Cell<u64> = const { Cell::new(0) };
    static SLICE_MS: Cell<u64> = const { Cell::new(0) };
    static YIELDED_MS: Cell<u64> = const { Cell::new(0) };
}

pub fn mk_yield() -> i64 {
    YIELDS.with(|y| y.set(y.get() + 1));
    let slice = SLICE_MS.with(Cell::get);
    YIELDED_MS.with(|t| t.set(t.get().saturating_add(slice)));
    0
}

/// How many times the driver yielded on this thread so far.
pub fn yields() -> u64 {
    YIELDS.with(Cell::get)
}

/// The milliseconds other work held the processor across this thread's
/// yields so far.
pub fn yielded_ms() -> u64 {
    YIELDED_MS.with(Cell::get)
}

/// Every yield from now on gives the processor away for `slice_ms`, until
/// the guard is dropped.
pub fn busy_scheduler(slice_ms: u64) -> Busy {
    SLICE_MS.with(|s| s.set(slice_ms));
    Busy
}

pub struct Busy;

impl Drop for Busy {
    fn drop(&mut self) {
        SLICE_MS.with(|s| s.set(0));
    }
}
