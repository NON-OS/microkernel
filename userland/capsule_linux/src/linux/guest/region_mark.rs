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

//! Marking file bytes that were never proved, and asking about them.

use super::handle::Guest;

impl Guest {
    /// Every region inside [at, at + len) holds file bytes nothing proved.
    pub fn mark_unproven(&mut self, at: u64, len: u64) {
        let end = at.saturating_add(len);
        for r in self.regions.iter_mut().filter(|r| r.at >= at && r.at < end) {
            r.unproven = true;
        }
    }

    /// Whether any region overlapping [at, at + len) is unproven file bytes.
    pub fn span_unproven(&self, at: u64, len: u64) -> bool {
        let end = at.saturating_add(len);
        self.regions.iter().any(|r| r.unproven && r.at < end && at < r.at.saturating_add(r.len))
    }
}
