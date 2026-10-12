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
//! The hub walk (nonos_usb_hub) run for this driver: a device below a hub
//! is classified as one on a root port is, and kept only when a HID
//! interface or a hub is bound on it.

use alloc::vec::Vec;

use crate::hub::{scan_hubs as walk, Classes, Hub, Place, XhciHost};

use super::classify::classify;
use super::devices::Devices;
use super::types::HidEndpoint;

pub(super) struct Hid<'a> {
    pub eps: &'a mut Vec<HidEndpoint>,
}

impl Classes<XhciHost> for Hid<'_> {
    fn classify(&mut self, h: &mut XhciHost, slot: u8, place: Place, hubs: &mut Vec<Hub>) -> bool {
        let _ = classify(h, slot, place, self.eps, hubs);
        self.eps.iter().any(|e| e.slot == slot)
    }

    fn forget(&mut self, slot: u8) {
        self.eps.retain(|e| e.slot != slot);
    }
}

/// Look at the ports of every hub held.
pub fn scan_hubs(xhci_port: u32, devs: &mut Devices) {
    let Devices { eps, hubs } = devs;
    walk(&mut XhciHost { port: xhci_port }, hubs, &mut Hid { eps });
}
