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

//! The hub class descriptor (USB 2.0 section 11.23.2.1, USB 3.2 section
//! 10.15.2.1). Only its fixed head is read: port count, characteristics and
//! the power-on delay. Pure, so the parse is proven on the host.

/// bDescriptorType of a USB 2 hub descriptor and of a SuperSpeed one.
pub const DT_HUB: u8 = 0x29;
pub const DT_SS_HUB: u8 = 0x2A;
/// The bytes asked for: up to bHubContrCurrent, the same in both layouts.
pub const HUB_HEAD_LEN: u16 = 7;
/// A route string has one 4-bit field per tier (xHCI 1.2 section 8.9), so a
/// port past 15 cannot be told apart from 15; a SuperSpeed hub has at most
/// 15 ports. Ports past this are left alone.
pub const MAX_HUB_PORTS: u8 = 15;
/// Linux `hub_power_on_good_delay`: at least 100 ms, whatever the hub says.
const MIN_POWER_ON_MS: u32 = 100;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HubDescriptor {
    /// bNbrPorts, capped at `MAX_HUB_PORTS`.
    pub ports: u8,
    /// bNbrPorts as the hub gave it.
    pub ports_named: u8,
    /// The TT think time code, wHubCharacteristics bits 6:5; 0 on a
    /// SuperSpeed hub, which has no TT.
    pub think_time: u8,
    /// How long after port power the port is usable, in milliseconds.
    pub power_on_ms: u32,
}

/// The head of a hub descriptor read from a hub that is SuperSpeed or not
/// (`superspeed`), or None when it is not one.
pub fn parse_hub_descriptor(buf: &[u8], superspeed: bool) -> Option<HubDescriptor> {
    let want = if superspeed { DT_SS_HUB } else { DT_HUB };
    if buf.len() < HUB_HEAD_LEN as usize || (buf[0] as u16) < HUB_HEAD_LEN || buf[1] != want {
        return None;
    }
    let named = buf[2];
    if named == 0 {
        return None;
    }
    let characteristics = u16::from_le_bytes([buf[3], buf[4]]);
    let think_time = if superspeed { 0 } else { ((characteristics >> 5) & 0x3) as u8 };
    Some(HubDescriptor {
        ports: named.min(MAX_HUB_PORTS),
        ports_named: named,
        think_time,
        power_on_ms: (buf[5] as u32 * 2).max(MIN_POWER_ON_MS),
    })
}
