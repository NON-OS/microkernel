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

//! Addressing one device and binding its HID interfaces, on a slot the
//! caller gives back if nothing binds.

use alloc::vec::Vec;

use crate::descriptors::hid_bindings;
use crate::xhci::{
    address_device, control_transfer, get_config_descriptor, PortSnapshot, XhciClientError,
};

use super::super::binding::configure_binding;
use super::constants::DESC_LEN;
use super::types::{HidEndpoint, Outcome};

pub(super) fn bind(
    xhci_port: u32,
    slot: u8,
    snap: PortSnapshot,
    out: &mut Vec<HidEndpoint>,
) -> Outcome {
    let dev = match address_device(xhci_port, slot, snap.port_id) {
        Ok(dev) => dev,
        Err(XhciClientError::Busy) => return Outcome::Busy,
        Err(_) => return Outcome::Failed,
    };
    if dev.slot_id != slot
        || dev.port_id != snap.port_id
        || dev.speed == 0
        || dev.max_packet_size == 0
    {
        return Outcome::Failed;
    }
    let mut desc = [0u8; DESC_LEN as usize];
    let Ok(len) = get_config_descriptor(xhci_port, slot, DESC_LEN, &mut desc) else {
        return Outcome::Failed;
    };
    let bindings = match hid_bindings(&desc[..len]) {
        Ok(b) if !b.is_empty() => b,
        _ => return Outcome::NotHid,
    };
    let mut dummy = [0u8; 0];
    if control_transfer(xhci_port, slot, 0x00, 0x09, 1, 0, 0, &mut dummy).is_err() {
        return Outcome::Failed;
    }
    for binding in bindings {
        configure_binding(xhci_port, slot, binding, out);
    }
    Outcome::Bound
}
