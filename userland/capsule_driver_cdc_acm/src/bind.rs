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
//! Whether a device is a serial line this driver serves, and setting it
//! up: the bulk pipes given to the host before the device is told to use
//! them, its configuration, the data interface's alternate, the
//! notification endpoint's ring, and 115200 8N1 where it takes a line
//! coding. DTR and RTS stay down until a client opens it, so a modem does
//! not answer before anyone listens.

use nonos_usbnet::desc::config_header;
use nonos_usbnet::say::say;
use nonos_usbnet::xhci::XhciBus;
use nonos_usbnet::{Bind, Bus, Found, Setup};

use crate::acm::{find_acm, set_line, AcmFunction, CAP_LINE, DEFAULT};
use crate::notify::ring;

pub struct Port {
    pub f: AcmFunction,
    pub ids: (u16, u16),
    /// The notification endpoint's DCI, when it has one the host took.
    pub notify: Option<u8>,
}

pub fn bind(mut bus: XhciBus, found: &Found) -> Bind<Port> {
    let pick = |raw: &alloc::vec::Vec<u8>| Some((config_header(raw)?.1, find_acm(raw)?));
    let Some((value, f)) = found.configs.iter().find_map(pick) else { return Bind::NotOurs };
    if let Err(e) = bus.configure_bulk(&f.pipes) {
        return Bind::Failed("bulk pipes refused", e);
    }
    if let Err(e) = bus.control_out(Setup::set_configuration(value), &[]) {
        return Bind::Failed("set configuration", e);
    }
    if f.data_alt != 0 {
        if let Err(e) = bus.control_out(Setup::set_interface(f.data, f.data_alt), &[]) {
            return Bind::Failed("set interface", e);
        }
    }
    // Without its notifications the line still moves bytes; only DCD,
    // DSR and the error events go unseen.
    let notify = f.notify.and_then(|n| ring(bus.xhci, bus.slot, &n).ok());
    if f.notify.is_some() && notify.is_none() {
        say(b"cdc-acm", &[b"notification endpoint refused, line state unseen"]);
    }
    if f.capabilities & CAP_LINE != 0 {
        if let Err(e) = bus.control_out(set_line(f.comm), &DEFAULT.bytes()) {
            return Bind::Failed("set line coding", e);
        }
    }
    Bind::Ours(Port { f, ids: (found.info.vendor, found.info.product), notify })
}
