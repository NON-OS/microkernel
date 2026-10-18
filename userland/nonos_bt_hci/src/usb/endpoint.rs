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

//! The endpoints of the Bluetooth interface. HCI events come on an
//! interrupt IN endpoint and ACL data on a bulk IN and a bulk OUT endpoint;
//! commands use the default control endpoint and need none of their own
//! (Vol 4, Part B, 2.1). The endpoint descriptor's layout is USB 2.0, 9.6.6.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Endpoint {
    /// bEndpointAddress: the number, with bit 7 set for IN.
    pub address: u8,
    /// wMaxPacketSize bits 10:0.
    pub max_packet: u16,
    pub interval: u8,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Role {
    Events,
    AclIn,
    AclOut,
}

const ATTR_BULK: u8 = 2;
const ATTR_INTERRUPT: u8 = 3;
const DIR_IN: u8 = 0x80;

/// An endpoint descriptor (at least 7 octets), and which of the three it
/// is; `None` for one the HCI transport does not use.
pub fn classify(d: &[u8]) -> Option<(Role, Endpoint)> {
    let &[_, _, address, attributes, mp0, mp1, interval, ..] = d else {
        return None;
    };
    let ep = Endpoint { address, max_packet: u16::from_le_bytes([mp0, mp1]) & 0x07ff, interval };
    let role = match (attributes & 3, address & DIR_IN != 0) {
        (ATTR_INTERRUPT, true) => Role::Events,
        (ATTR_BULK, true) => Role::AclIn,
        (ATTR_BULK, false) => Role::AclOut,
        _ => return None,
    };
    (address & 0x0f != 0 && ep.max_packet != 0).then_some((role, ep))
}
