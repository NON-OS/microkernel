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

//! Addressing the device on a port, reading what it offers, and asking the
//! driver whether it is its own; each step that fails says so on the log.

use super::bound::Bound;
use super::probe::Probe;
use crate::bind::Bind;
use crate::found::fetch;
use crate::run::BindFn;
use crate::say::say_port;
use crate::xhci::{address_device, XhciBus, E_BUSY};

pub(super) fn classify<N>(mut bus: XhciBus, port: u8, tag: &[u8], bind: BindFn<N>) -> Probe<N> {
    match address_device(bus.xhci, bus.slot, port) {
        Ok(()) => {}
        Err(E_BUSY) => return Probe::Busy,
        Err(e) => {
            say_port(tag, port, (0, 0), b"address failed", Some(e));
            return Probe::Failed;
        }
    }
    let found = match fetch(&mut bus, port) {
        Ok(found) => found,
        Err(e) => {
            say_port(tag, port, (0, 0), b"descriptors unread", Some(e));
            return Probe::Failed;
        }
    };
    let ids = (found.info.vendor, found.info.product);
    match bind(bus, &found) {
        Bind::Ours(nic) => {
            say_port(tag, port, ids, b"bound", None);
            Probe::Ours(Bound { nic, xhci: bus.xhci, slot: bus.slot, port })
        }
        Bind::NotOurs => {
            say_port(tag, port, ids, b"not this driver's device", None);
            Probe::NotOurs
        }
        Bind::Failed(what, e) => {
            say_port(tag, port, ids, what.as_bytes(), Some(e));
            Probe::Failed
        }
    }
}
