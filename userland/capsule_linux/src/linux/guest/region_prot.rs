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

//! Recording a protection change on the spans a guest holds.

use alloc::vec::Vec;

use super::handle::Guest;
use super::region::Region;
use super::region_cut::cut;

impl Guest {
    /// Every backed part of `[at, at + len)` now has this protection. The
    /// list is what fork maps the child from, so a list that kept the old
    /// protection would give the child access the parent gave up.
    pub fn set_prot(&mut self, at: u64, len: u64, write: bool, exec: bool, access: bool) {
        let end = at.saturating_add(len);
        let changed: Vec<Region> = self
            .regions
            .iter()
            .filter(|r| r.backed && r.at < end && at < r.at.saturating_add(r.len))
            .map(|r| {
                let from = r.at.max(at);
                let to = r.at.saturating_add(r.len).min(end);
                Region { at: from, len: to - from, write, exec, access, ..*r }
            })
            .collect();
        for piece in &changed {
            self.regions = cut(&self.regions, piece.at, piece.len);
        }
        self.regions.extend(changed);
    }
}
