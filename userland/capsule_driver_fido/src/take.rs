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
//! The key among a device's candidate interfaces taken: the one whose
//! report descriptor is FIDO claimed with the host, so a driver sharing the
//! device never takes it too, and brought up (up.rs).

use alloc::vec::Vec;

use nonos_usbnet::xhci::{claim_interface, disable_slot, XhciBus, E_BUSY};
use nonos_usbnet::{Bind, Bus, Found, Setup};

use crate::application::application;
use crate::bind::Port;
use crate::ctaphid::Key;
use crate::up::up;

const FIDO: (u16, u16) = (0xF1D0, 0x01);
const REPORT_DESCRIPTOR: u8 = 0x22;

pub fn take(mut bus: XhciBus, found: &Found, keys: Vec<Key>, shared: bool) -> Bind<Port> {
    let port = found.port;
    let mut key = None;
    for k in keys {
        // On a shared device the host serves claimants only, so a candidate
        // is claimed before it is asked, and given back if it is not a key.
        if shared && claim_interface(bus.xhci, bus.slot, port, k.interface).is_err() {
            continue;
        }
        if is_fido(&mut bus, k.interface) {
            key = Some(k);
            break;
        }
        if shared {
            disable_slot(bus.xhci, bus.slot);
        }
    }
    let Some(key) = key else { return Bind::NotOurs };
    // A host that keeps no claims still serves an unshared device.
    if !shared && claim_interface(bus.xhci, bus.slot, port, key.interface) == Err(E_BUSY) {
        return Bind::NotOurs;
    }
    match up(&mut bus, &key) {
        Ok((dci, ch)) => {
            let ids = (found.info.vendor, found.info.product);
            Bind::Ours(Port { ids, dci, ch, shared })
        }
        Err((step, e)) => {
            // Joined, our claim goes back; alone, the search frees the slot.
            if shared {
                disable_slot(bus.xhci, bus.slot);
            }
            Bind::Failed(step, e)
        }
    }
}

fn is_fido(bus: &mut XhciBus, interface: u8) -> bool {
    let mut report = [0u8; 256];
    let get = Setup::new(0x81, 0x06, (REPORT_DESCRIPTOR as u16) << 8, interface as u16);
    bus.control_in(get, &mut report).is_ok_and(|n| application(&report[..n]) == Some(FIDO))
}
