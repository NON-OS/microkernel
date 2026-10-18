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
//! The devices addressed below a root port, by their route string.
use super::super::SlotResources;
use super::types::SlotTable;

impl SlotTable {
    /// The addressed slot on `root_port` at `route`, if any.
    pub fn at_route(&self, root_port: u8, route: u32) -> Option<&SlotResources> {
        self.resources.iter().find(|r| {
            r.port_id == root_port && r.topo.route == route && self.addressed[r.slot_id as usize]
        })
    }

    /// The hub on `root_port` at `route`: addressed there, and set up by
    /// Configure Hub.
    pub fn hub_at(&self, root_port: u8, route: u32) -> Option<&SlotResources> {
        self.at_route(root_port, route).filter(|r| r.hub.is_some())
    }
}
