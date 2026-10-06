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

//! A kept device that is gone: its endpoints are dropped, a hub takes every
//! device below it along, and the slot is given back to the controller.

use crate::xhci::disable_slot;

use super::devices::Devices;

pub(super) fn forget_slot(xhci_port: u32, slot: u8, devs: &mut Devices) {
    devs.eps.retain(|e| e.slot != slot);
    if let Some(i) = devs.hubs.iter().position(|h| h.slot == slot) {
        let hub = devs.hubs.swap_remove(i);
        for &child in hub.child.iter().filter(|&&c| c != 0) {
            forget_slot(xhci_port, child, devs);
        }
    }
    disable_slot(xhci_port, slot);
}
