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

//! The wait after wrong entries. The first `FREE_TRIES` cost nothing; each
//! one after that doubles the wait before the next attempt is taken, from
//! `BASE_MS` up to `MAX_WAIT_MS`. The count is kept in the record, so a
//! restart does not clear it: the wait it calls for runs again from boot.

pub const FREE_TRIES: u32 = 3;
pub const BASE_MS: u64 = 2_000;
pub const MAX_WAIT_MS: u64 = 15 * 60 * 1000;

/// The wait before the next attempt after `failures` wrong entries in a row.
pub fn wait_ms(failures: u32) -> u64 {
    if failures <= FREE_TRIES {
        return 0;
    }
    let doublings = failures - FREE_TRIES - 1;
    if doublings >= 20 {
        return MAX_WAIT_MS;
    }
    (BASE_MS << doublings).min(MAX_WAIT_MS)
}

/// The wrong entries and the time the next attempt is taken.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Gate {
    failures: u32,
    open_at_ms: i64,
}

impl Gate {
    /// The gate for a count read back (at boot, or with a record set):
    /// its whole wait runs from `now_ms`.
    pub fn new(failures: u32, now_ms: i64) -> Gate {
        Gate { failures, open_at_ms: later(now_ms, wait_ms(failures)) }
    }

    pub fn failures(&self) -> u32 {
        self.failures
    }

    /// Milliseconds before an attempt is taken; 0 now.
    pub fn wait_left(&self, now_ms: i64) -> u64 {
        u64::try_from(self.open_at_ms.saturating_sub(now_ms)).unwrap_or(0)
    }

    /// One more wrong entry at `now_ms`.
    pub fn failed(&mut self, now_ms: i64) {
        self.failures = self.failures.saturating_add(1);
        self.open_at_ms = later(now_ms, wait_ms(self.failures));
    }

    /// The right passphrase: the count starts again.
    pub fn passed(&mut self) {
        self.failures = 0;
        self.open_at_ms = 0;
    }
}

fn later(now_ms: i64, wait: u64) -> i64 {
    now_ms.saturating_add(i64::try_from(wait).unwrap_or(i64::MAX))
}
