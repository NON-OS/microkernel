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

//! A root port whose device is gone. Its tries start again, so whatever is
//! plugged in next is looked at, and the slot of a device bound there is
//! given back: left addressed, the port stayed claimed in the controller
//! driver and the next device on it was never addressed. A hub there takes
//! every device below it along.

use crate::xhci::disable_slot;

use super::devices::Devices;
use super::drop_port::drop_port;
use super::forget_slot::forget_slot;
use super::say_port::say_port;

pub(super) fn forget(xhci_port: u32, port: u8, tried: &mut u8, devs: &mut Devices) {
    *tried = 0;
    let top_hub = |d: &Devices| {
        d.hubs.iter().find(|h| h.place.root_port == port && h.place.depth == 0).map(|h| h.slot)
    };
    while let Some(slot) = top_hub(devs) {
        forget_slot(xhci_port, slot, devs);
        say_port(port, b"hub unplugged, it and every device below it are given back");
    }
    let slots = drop_port(&mut devs.eps, port);
    for &slot in &slots {
        disable_slot(xhci_port, slot);
    }
    if !slots.is_empty() {
        say_port(port, b"HID device unplugged, its slot is given back");
    }
}
