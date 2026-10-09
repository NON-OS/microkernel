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

//! The uptime clock, virtual: every read moves it on one millisecond, so a
//! wait bounded by a deadline ends after a fixed number of polls however
//! fast the host is, and a test can read how long the driver waited.

use std::cell::Cell;

thread_local! {
    static NOW_MS: Cell<u64> = const { Cell::new(0) };
}

pub fn mk_uptime_ms() -> i64 {
    NOW_MS.with(|n| {
        n.set(n.get() + 1);
        n.get() as i64
    })
}

/// The shipping `nonos_libc::Deadline`, against the virtual clock.
#[derive(Clone, Copy)]
pub struct Deadline {
    end_ms: u64,
}

impl Deadline {
    pub fn after_ms(timeout_ms: u64) -> Self {
        Self { end_ms: (mk_uptime_ms() as u64).saturating_add(timeout_ms) }
    }

    pub fn expired(&self) -> bool {
        mk_uptime_ms() as u64 >= self.end_ms
    }
}
