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

//! The requests that bind a device: every pipe configured for a class
//! driver, a hub's configuration, and SET_CONFIGURATION, which turns the
//! device's interfaces on (USB 2.0 9.4.7). Descriptor reads, the claim and
//! the transfers on pipes already configured are not binds.

use crate::protocol::*;

const SET_CONFIGURATION: u8 = 0x09;

pub fn binds(op: u16, body: &[u8]) -> bool {
    match op {
        OP_ALLOC_TRANSFER_RING
        | OP_CONFIGURE_BULK
        | OP_CONFIGURE_BULK2
        | OP_CONFIGURE_STREAMS
        | OP_CONFIGURE_ISOCH
        | OP_CONFIGURE_HUB => true,
        // slot, then the setup packet: bmRequestType, bRequest, ...
        OP_CONTROL_TRANSFER => body.get(2..4) == Some(&[0x00, SET_CONFIGURATION][..]),
        _ => false,
    }
}

/// The configuration value a SET_CONFIGURATION sets (wValue's low byte).
pub fn set_configuration(op: u16, body: &[u8]) -> Option<u8> {
    let set = op == OP_CONTROL_TRANSFER && body.get(2..4) == Some(&[0x00, SET_CONFIGURATION][..]);
    set.then(|| body.get(4).copied()).flatten()
}
