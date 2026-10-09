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
//! Who holds a root port, as `OP_PORT_STATUS` reports it and as
//! `OP_ADDRESS_DEVICE` checks it: one class driver per device, and a port
//! another driver is still classifying is not reset under it.
use super::types::SlotTable;
/// No slot is addressed on the port.
pub const PORT_FREE: u8 = 0;
/// A slot is addressed on the port with no endpoint of its own yet: a
/// class driver is reading its descriptors and may still let it go.
pub const PORT_ADDRESSED: u8 = 1;
/// A class driver configured an interrupt or bulk endpoint on it.
pub const PORT_CLAIMED: u8 = 2;
impl SlotTable {
    pub fn port_state(&self, port_id: u8) -> u8 {
        let state = |r: &super::super::SlotResources| {
            if !r.interrupt.is_empty() || r.bulk.is_some() {
                PORT_CLAIMED
            } else {
                PORT_ADDRESSED
            }
        };
        self.resources.iter().filter(|r| r.port_id == port_id).map(state).max().unwrap_or(PORT_FREE)
    }
}
