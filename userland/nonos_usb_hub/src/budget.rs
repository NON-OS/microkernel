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

//! How often one port, of a hub or of the root hub, may be tried:
//! TRIES_PER_MINUTE in any minute, counted across disconnects, so a port
//! that reconnects as fast as it is looked at is still tried only that
//! often.

pub const TRIES_PER_MINUTE: u8 = 3;
pub(super) const MINUTE_MS: u64 = 60_000;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Budget {
    pub(super) tries: u8,
    pub(super) since_ms: u64,
}

impl Budget {
    pub const fn new() -> Self {
        Self { tries: 0, since_ms: 0 }
    }

    /// Spend one try at `now_ms`, if the minute since the first still has
    /// one left.
    pub fn spend(&mut self, now_ms: u64) -> bool {
        if self.tries == 0 || now_ms.saturating_sub(self.since_ms) >= MINUTE_MS {
            (self.tries, self.since_ms) = (0, now_ms);
        }
        self.tries = self.tries.saturating_add(1);
        self.tries <= TRIES_PER_MINUTE
    }
}
