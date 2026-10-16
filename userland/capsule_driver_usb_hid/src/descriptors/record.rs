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

//! An interface record and an endpoint record of a configuration, read
//! into a binding candidate.

use alloc::vec::Vec;

use super::binding::HidBinding;
use super::types::{Endpoint, Interface};

pub(super) fn parse_interface(buf: &[u8]) -> Option<Interface> {
    Some(Interface { number: buf[2], class: buf[5], subclass: buf[6], protocol: buf[7] })
}

pub(super) fn maybe_push(out: &mut Vec<HidBinding>, iface: Option<Interface>, buf: &[u8]) {
    let Some(iface) = iface else { return };
    let raw = u16::from_le_bytes([buf[4], buf[5]]);
    // Bits 15:13 are reserved (USB 2.0 9.6.6, as recalled, unverified): no
    // report endpoint sets them. Bits 12:11 count transactions and are read
    // past, as the size is bits 10:0.
    if raw >> 13 != 0 {
        return;
    }
    let ep = Endpoint {
        address: buf[2],
        attributes: buf[3],
        max_packet_size: raw & 0x07ff,
        interval: buf[6],
    };
    if let Some(binding) = HidBinding::from_pair(iface, ep) {
        out.push(binding);
    }
}
