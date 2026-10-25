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
//! Whether a device is a security key, and setting it up: configured, each
//! HID interface that could be one asked for its report descriptor, and
//! the one whose application is FIDO (usage page F1D0h, usage 1, as
//! recalled, unverified) taken (take.rs). A composite key whose keyboard
//! usb_hid0 bound is joined instead (`share`): never configured again, and
//! only its interfaces no other driver holds are looked at.

use nonos_usbnet::desc::config_header;
use nonos_usbnet::xhci::XhciBus;
use nonos_usbnet::{Bind, Bus, Found, Setup};

use crate::ctaphid::{find_keys, free_keys, Channel};
use crate::take::take;

pub struct Port {
    pub ids: (u16, u16),
    /// The IN and OUT endpoints' DCIs.
    pub dci: (u8, u8),
    pub ch: Channel,
    /// Joined beside another driver, which keeps the device configured.
    pub shared: bool,
}

pub fn bind(mut bus: XhciBus, found: &Found) -> Bind<Port> {
    let pick = |raw: &alloc::vec::Vec<u8>| Some((config_header(raw)?.1, find_keys(raw)));
    let Some((value, keys)) = found.configs.iter().filter_map(pick).find(|k| !k.1.is_empty())
    else {
        return Bind::NotOurs;
    };
    if let Err(e) = bus.control_out(Setup::set_configuration(value), &[]) {
        return Bind::Failed("set configuration", e);
    }
    take(bus, found, keys, false)
}

/// A device another driver configured: its first configuration, the one a
/// HID driver sets, and of it only the interfaces `claimed` leaves free.
pub fn share(bus: XhciBus, found: &Found, claimed: u32) -> Bind<Port> {
    let Some(raw) = found.configs.first() else { return Bind::NotOurs };
    let keys = free_keys(raw, claimed);
    if keys.is_empty() {
        return Bind::NotOurs;
    }
    take(bus, found, keys, true)
}
