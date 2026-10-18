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

//! The kernel uptime, on a clock the test's model keeps to as well.

use std::cell::Cell;
use std::rc::Rc;

thread_local! {
    static NOW: Rc<Cell<u64>> = Rc::new(Cell::new(0));
}

/// Milliseconds since boot. Each read moves the clock on by one, so every
/// bounded wait ends, and its bound is measured in reads.
pub fn mk_uptime_ms() -> i64 {
    NOW.with(|n| {
        n.set(n.get() + 1);
        n.get() as i64
    })
}

/// This thread's clock, for a model that keeps to the same time.
pub fn uptime() -> Rc<Cell<u64>> {
    NOW.with(Rc::clone)
}
