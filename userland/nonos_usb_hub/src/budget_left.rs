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

//! Whether a port's budget has a try left, asked before a hub port's turn
//! begins. Its own file because the root-port scan, which mounts budget.rs,
//! only spends.

use super::budget::{Budget, MINUTE_MS, TRIES_PER_MINUTE};

impl Budget {
    /// Whether a try is left at `now_ms`, spending none.
    pub fn left(&self, now_ms: u64) -> bool {
        let fresh = self.tries == 0 || now_ms.saturating_sub(self.since_ms) >= MINUTE_MS;
        fresh || self.tries < TRIES_PER_MINUTE
    }
}
