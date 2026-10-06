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

//! The two hub class reads: the hub descriptor and one port's status (USB
//! 2.0 section 11.24.2.5 and 11.24.2.7).

use crate::xhci::control_transfer;

use super::descriptor::{parse_hub_descriptor, HubDescriptor, DT_HUB, DT_SS_HUB, HUB_HEAD_LEN};
use super::port_status::{decode_port_status, HubPort};

const GET_STATUS: u8 = 0x00;
const GET_DESCRIPTOR: u8 = 0x06;
/// Class requests to the hub itself, and to one of its ports.
const RT_HUB_IN: u8 = 0xA0;
const RT_PORT_IN: u8 = 0xA3;

pub fn hub_descriptor(xhci_port: u32, slot: u8, superspeed: bool) -> Option<HubDescriptor> {
    let kind = if superspeed { DT_SS_HUB } else { DT_HUB };
    let mut buf = [0u8; HUB_HEAD_LEN as usize];
    let value = (kind as u16) << 8;
    let n = control_transfer(
        xhci_port,
        slot,
        RT_HUB_IN,
        GET_DESCRIPTOR,
        value,
        0,
        HUB_HEAD_LEN,
        &mut buf,
    )
    .ok()?;
    parse_hub_descriptor(&buf[..n], superspeed)
}

pub fn port_status(xhci_port: u32, slot: u8, port: u8, superspeed: bool) -> Option<HubPort> {
    let mut buf = [0u8; 4];
    let n = control_transfer(xhci_port, slot, RT_PORT_IN, GET_STATUS, 0, port as u16, 4, &mut buf)
        .ok()?;
    (n == 4).then(|| decode_port_status(buf, superspeed))
}
