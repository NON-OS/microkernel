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

//! The hold ecm/claim.rs asks for, made with driver.xhci0's claim table:
//! the device's slot, the port it sits on, and the interface.

use nonos_usbnet::xhci::{claim_interface, XhciBus};

use crate::ecm::claim::Claim;

impl Claim for XhciBus {
    fn claim(&mut self, port: u8, interface: u8) -> Result<(), i32> {
        claim_interface(self.xhci, self.slot, port, interface)
    }
}
