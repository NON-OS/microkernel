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
//! Which addressed slots are hubs: a device below a hub is addressed only
//! on a root port a hub holds, and its TT only in a hub's slot.

use super::types::SlotTable;

impl SlotTable {
    /// Whether `slot_id` is addressed and marked a hub.
    pub fn is_hub(&self, slot_id: u8) -> bool {
        self.resources.iter().any(|r| r.slot_id == slot_id && r.hub)
    }

    /// Whether a hub is addressed on root port `port_id`.
    pub fn hub_on(&self, port_id: u8) -> bool {
        self.resources.iter().any(|r| r.port_id == port_id && r.hub)
    }
}
