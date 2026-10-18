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
//! Which addressed slots are hubs (a child is parked only below one), and
//! how held a slot is, for the port list's virtual ports.

use super::types::SlotTable;

impl SlotTable {
    /// Whether `slot_id` is addressed and marked a hub.
    pub fn is_hub(&self, slot_id: u8) -> bool {
        self.resources.iter().any(|r| r.slot_id == slot_id && r.hub.is_some())
    }

    /// The owner byte of addressed `slot_id`: 2 once a class driver
    /// configured an endpoint on it, else 1.
    pub fn held(&self, slot_id: u8) -> u8 {
        let claimed = |r: &&super::super::SlotResources| !r.interrupt.is_empty() || r.has_bulk();
        let found = self.resources.iter().find(|r| r.slot_id == slot_id);
        if found.as_ref().is_some_and(claimed) {
            2
        } else {
            1
        }
    }
}
