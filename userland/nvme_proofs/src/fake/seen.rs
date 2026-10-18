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

//! What the fake controller saw, for the tests to check.

use super::Up;
use crate::admin::Submission;

impl Up {
    /// The admin commands fetched since the first `from`.
    pub fn admin_since(&self, from: usize) -> Vec<Submission> {
        self.fake.borrow().admin_seen.get(from..).unwrap_or_default().to_vec()
    }

    /// Their opcodes.
    pub fn ops_since(&self, from: usize) -> Vec<u32> {
        self.admin_since(from).iter().map(|c| c.cdw0 & 0xff).collect()
    }

    /// How many admin commands were fetched so far.
    pub fn admin_count(&self) -> usize {
        self.fake.borrow().admin_seen.len()
    }
}

/// How many times the controller was disabled.
pub(crate) fn disables() -> usize {
    nonos_libc::calls().iter().filter(|c| c.0 == "disabled").count()
}
