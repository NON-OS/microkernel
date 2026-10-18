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
//! Whether a device is a reader this driver serves, and setting it up: the
//! bulk pipes given to the host before the device is told to use them, its
//! configuration, and the interrupt endpoint's ring for slot changes. A
//! reader that wants T=0 or T=1 done by the host is named once and left.

use nonos_usbnet::desc::config_header;
use nonos_usbnet::say::say;
use nonos_usbnet::xhci::XhciBus;
use nonos_usbnet::{Bind, Bus, Found, Setup};

use crate::ccid::{find_reader, Reader};

pub struct Port {
    pub r: Reader,
    pub ids: (u16, u16),
    /// The slot-change endpoint's DCI, when it has one the host took.
    pub notify: Option<u8>,
}

pub fn bind(mut bus: XhciBus, found: &Found) -> Bind<Port> {
    let pick = |raw: &alloc::vec::Vec<u8>| Some((config_header(raw)?.1, find_reader(raw)?));
    let Some((value, r)) = found.configs.iter().find_map(pick) else { return Bind::NotOurs };
    let r = match r {
        Ok(r) => r,
        Err(why) => {
            say(b"ccid", &[b"reader left: ", why.as_bytes()]);
            return Bind::NotOurs;
        }
    };
    if let Err(e) = bus.configure_bulk(&r.pipes) {
        return Bind::Failed("bulk pipes refused", e);
    }
    if let Err(e) = bus.control_out(Setup::set_configuration(value), &[]) {
        return Bind::Failed("set configuration", e);
    }
    // Without slot changes the reader still answers; only a card swapped
    // between two STATUS requests goes unseen.
    let notify = r.notify.and_then(|n| bus.configure_interrupt(n).ok());
    if r.notify.is_some() && notify.is_none() {
        say(b"ccid", &[b"slot change endpoint refused, swaps unseen"]);
    }
    Bind::Ours(Port { r, ids: (found.info.vendor, found.info.product), notify })
}
