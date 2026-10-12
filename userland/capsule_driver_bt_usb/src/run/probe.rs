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

//! One look at a device on a free port: addressed, its descriptors read,
//! and bound if it is a Bluetooth controller.

use nonos_usbnet::found::fetch;
use nonos_usbnet::xhci::{address_device, XhciBus, E_BUSY};
use nonos_usbnet::Bind;

use super::desk::Desk;
use super::say::say;
use crate::usb::{bind, Bound};

pub enum Probe {
    Ours(Bound),
    NotOurs,
    /// Another class driver is addressing the port; ask again next pass.
    Busy,
    Failed,
}

pub fn probe(mut bus: XhciBus, port: u8, desk: &Desk) -> Probe {
    match address_device(bus.xhci, bus.slot, port) {
        Ok(()) => {}
        Err(E_BUSY) => return Probe::Busy,
        Err(_) => return failed("address failed"),
    }
    let found = match fetch(&mut bus, port) {
        Ok(found) => found,
        Err(_) => return failed("descriptors unread"),
    };
    match bind(bus, &found, desk.clone()) {
        Bind::Ours(bound) => Probe::Ours(bound),
        Bind::NotOurs => Probe::NotOurs,
        Bind::Failed(why, _) => failed(why),
    }
}

fn failed(why: &str) -> Probe {
    say(&[b"controller not usable: ", why.as_bytes()]);
    Probe::Failed
}
