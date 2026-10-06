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

//! Addressing one device on a root port, on a slot the caller gives back
//! if nothing binds.

use crate::hub::Place;
use crate::xhci::{address_device, PortSnapshot, XhciClientError};

use super::classify::classify;
use super::devices::Devices;
use super::types::Outcome;

pub(super) fn bind(xhci_port: u32, slot: u8, snap: PortSnapshot, devs: &mut Devices) -> Outcome {
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
    classify(xhci_port, slot, Place::root(snap.port_id, dev.speed), devs)
}
