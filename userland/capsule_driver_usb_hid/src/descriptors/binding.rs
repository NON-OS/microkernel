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

use super::packet_size::holds_a_report;
use super::types::{
    Endpoint, HidKind, Interface, CLASS_HID, EP_TRANSFER_INTERRUPT, PROTOCOL_KEYBOARD,
    PROTOCOL_MOUSE, SUBCLASS_BOOT,
};

#[derive(Debug, Clone, Copy)]
pub struct HidBinding {
    pub kind: HidKind,
    pub interface_number: u8,
    pub endpoint_address: u8,
    pub interval: u8,
    pub max_packet_size: u16,
}

impl HidBinding {
    pub fn from_pair(iface: Interface, ep: Endpoint) -> Option<Self> {
        if iface.class != CLASS_HID || !is_interrupt_in(ep) {
            return None;
        }
        let kind = if iface.subclass == SUBCLASS_BOOT {
            match iface.protocol {
                PROTOCOL_KEYBOARD => HidKind::Keyboard,
                PROTOCOL_MOUSE => HidKind::Mouse,
                _ => return None,
            }
        } else {
            HidKind::Tablet
        };
        if !holds_a_report(kind, ep.max_packet_size) {
            return None;
        }
        Some(Self {
            kind,
            interface_number: iface.number,
            endpoint_address: ep.address,
            interval: ep.interval,
            max_packet_size: ep.max_packet_size,
        })
    }
}

/// An IN endpoint of interrupt type, other than endpoint 0: that is the
/// control pipe every device has, and the controller driver would turn a
/// binding of it into a reconfiguration of that pipe.
fn is_interrupt_in(ep: Endpoint) -> bool {
    (ep.address & 0x80) != 0
        && (ep.address & 0x0f) != 0
        && (ep.attributes & 0x03) == EP_TRANSFER_INTERRUPT
}
