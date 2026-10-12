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
//! recalled, unverified) given its two interrupt rings and a channel.

use nonos_usbnet::desc::config_header;
use nonos_usbnet::xhci::XhciBus;
use nonos_usbnet::{Bind, Bus, Found, Setup};

use crate::application::application;
use crate::ctaphid::{find_keys, Channel};
use crate::up::up;

const FIDO: (u16, u16) = (0xF1D0, 0x01);
const REPORT_DESCRIPTOR: u8 = 0x22;

pub struct Port {
    pub ids: (u16, u16),
    /// The IN and OUT endpoints' DCIs.
    pub dci: (u8, u8),
    pub ch: Channel,
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
    let Some(key) = keys.into_iter().find(|k| is_fido(&mut bus, k.interface)) else {
        return Bind::NotOurs;
    };
    match up(&mut bus, &key) {
        Ok((dci, ch)) => Bind::Ours(Port { ids: (found.info.vendor, found.info.product), dci, ch }),
        Err((step, e)) => Bind::Failed(step, e),
    }
}

fn is_fido(bus: &mut XhciBus, interface: u8) -> bool {
    let mut report = [0u8; 256];
    let get = Setup::new(0x81, 0x06, (REPORT_DESCRIPTOR as u16) << 8, interface as u16);
    bus.control_in(get, &mut report).is_ok_and(|n| application(&report[..n]) == Some(FIDO))
}
