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


//! The uptime clock, virtual. Every read moves it on one millisecond and
//! every sleep by its length, so a driver loop bounded by a deadline ends
//! after a fixed number of polls on the host, however fast the host runs,
//! and a test can tell a bound in time from a bound in iterations.

use std::cell::Cell;

use crate::misc::slept_ms;

thread_local! {
    static READS: Cell<u64> = const { Cell::new(0) };
}

pub fn mk_uptime_ms() -> i64 {
    let reads = READS.with(|r| {
        r.set(r.get() + 1);
        r.get()
    });
    (reads + slept_ms()) as i64
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

    pub const fn at(end_ms: u64) -> Self {
        Self { end_ms }
    }

    pub const fn is_past(&self, now_ms: u64) -> bool {
        now_ms >= self.end_ms
    }

    pub fn expired(&self) -> bool {
        self.is_past(mk_uptime_ms() as u64)
    }
}
