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

use super::entry::TrayEntry;
use super::table::TrayTable;

impl TrayTable {
    /// How many items `owner_pid` holds.
    pub fn held_by(&self, owner_pid: u32) -> usize {
        self.entries.iter().filter(|e| e.in_use && e.owner_pid == owner_pid).count()
    }

    /// Remove every item whose owner `alive` says has ended, and say how many
    /// were removed. Only the owner removes its item, so one that ended left
    /// its item, and its place, for good.
    pub fn remove_ended(&mut self, alive: impl Fn(u32) -> bool) -> usize {
        let mut removed = 0;
        for slot in self.entries.iter_mut() {
            if slot.in_use && !alive(slot.owner_pid) {
                *slot = TrayEntry::default();
                removed += 1;
            }
        }
        removed
    }
}
