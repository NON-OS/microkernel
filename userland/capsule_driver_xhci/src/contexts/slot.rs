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

//! The Slot Context (xHCI 1.2 section 6.2.2) as Address Device first
//! writes it, and the hub fields Configure Endpoint adds to it.

use super::route::Topology;

/// Context Entries is 1 at Address Device: EP0 is the only endpoint.
const ONE_ENTRY: u32 = 1 << 27;
const SPEED_SHIFT: u32 = 20;
const MTT: u32 = 1 << 25;
const HUB: u32 = 1 << 26;
const ROOT_PORT_SHIFT: u32 = 16;
const PORTS_SHIFT: u32 = 24;
const TT_PORT_SHIFT: u32 = 8;
const TTT_SHIFT: u32 = 16;

/// Where Address Device places a device: its root port, its speed id and
/// its place below that port.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Place {
    pub root_port: u8,
    pub speed: u8,
    pub topo: Topology,
}

/// Slot Context dwords 0 to 2 for a device at `place`.
pub fn slot_dwords(place: &Place) -> [u32; 3] {
    let t = place.topo;
    let mtt = if t.mtt && t.tt_slot != 0 { MTT } else { 0 };
    let dw0 = (t.route & 0xF_FFFF) | ((place.speed as u32 & 0xF) << SPEED_SHIFT) | mtt | ONE_ENTRY;
    let dw1 = (place.root_port as u32) << ROOT_PORT_SHIFT;
    let dw2 = t.tt_slot as u32 | (t.tt_port as u32) << TT_PORT_SHIFT;
    [dw0, dw1, dw2]
}

/// What Configure Hub tells the controller of a hub (xHCI 1.2 section
/// 6.2.2, Linux `xhci_update_hub_device`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct HubInfo {
    /// Downstream ports, 1 to `MAX_HUB_PORTS`.
    pub ports: u8,
    /// TT Think Time, 0 to 3; written only for a high-speed hub.
    pub think_time: u8,
    /// Multi-TT; written only for a high-speed hub.
    pub mtt: bool,
}

/// Slot Context dwords 0 to 2 with the hub fields of `hub` set.
pub fn hub_dwords(dw: [u32; 3], hub: HubInfo, high_speed: bool) -> [u32; 3] {
    let tt = |on: u32| if high_speed { on } else { 0 };
    let dw0 = (dw[0] & !MTT) | HUB | tt(if hub.mtt { MTT } else { 0 });
    let dw1 = (dw[1] & !(0xFF << PORTS_SHIFT)) | (hub.ports as u32) << PORTS_SHIFT;
    let dw2 = (dw[2] & !(0x3 << TTT_SHIFT)) | tt((hub.think_time as u32 & 0x3) << TTT_SHIFT);
    [dw0, dw1, dw2]
}
