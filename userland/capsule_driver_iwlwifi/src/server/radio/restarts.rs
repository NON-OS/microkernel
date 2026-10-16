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

//! How often the firmware is restarted after it raised its error cause: three
//! tries, after 1 s, 10 s and 60 s; the count starts again once the radio has
//! run ten minutes without one. A radio whose tries are spent stays down.

/// The rests before each try; their count is the bound.
const RESTS_MS: [u64; 3] = [1_000, 10_000, 60_000];
/// A failure this long after the last one starts the count again.
pub const FORGIVE_MS: u64 = 600_000;

/// The restarts tried since the count last started.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Restarts {
    tries: u8,
    last_ms: Option<u64>,
}

impl Restarts {
    pub const NEW: Self = Self { tries: 0, last_ms: None };

    /// The restarts tried so far.
    pub fn tries(&self) -> u8 {
        self.tries
    }

    /// A failure at `now_ms`: when to try the next restart, or `None` when
    /// the tries are spent.
    pub fn after_failure(&mut self, now_ms: u64) -> Option<u64> {
        if self.last_ms.is_some_and(|t| now_ms.saturating_sub(t) >= FORGIVE_MS) {
            self.tries = 0;
        }
        let rest = *RESTS_MS.get(usize::from(self.tries))?;
        self.tries = self.tries.saturating_add(1);
        self.last_ms = Some(now_ms);
        Some(now_ms.saturating_add(rest))
    }
}
