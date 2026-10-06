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

//! The hub class writes: a port feature set or cleared (USB 2.0 section
//! 11.24.2.2 and 11.24.2.13) and a SuperSpeed hub's depth (USB 3.2 section
//! 10.16.2.9). Each says whether the hub took it.

use crate::xhci::control_transfer;

const CLEAR_FEATURE: u8 = 0x01;
const SET_FEATURE: u8 = 0x03;
const SET_HUB_DEPTH: u8 = 0x0C;
const RT_HUB_OUT: u8 = 0x20;
const RT_PORT_OUT: u8 = 0x23;

/// Port feature selectors (USB 2.0 table 11-17).
pub const PORT_RESET: u16 = 4;
pub const PORT_POWER: u16 = 8;
pub const C_PORT_CONNECTION: u16 = 16;
pub const C_PORT_RESET: u16 = 20;

pub fn set_port_feature(xhci_port: u32, slot: u8, port: u8, feature: u16) -> bool {
    let mut none = [0u8; 0];
    control_transfer(xhci_port, slot, RT_PORT_OUT, SET_FEATURE, feature, port as u16, 0, &mut none)
        .is_ok()
}

pub fn clear_port_feature(xhci_port: u32, slot: u8, port: u8, feature: u16) -> bool {
    let mut none = [0u8; 0];
    control_transfer(
        xhci_port,
        slot,
        RT_PORT_OUT,
        CLEAR_FEATURE,
        feature,
        port as u16,
        0,
        &mut none,
    )
    .is_ok()
}

/// A SuperSpeed hub reads its own field of the route string by its depth:
/// 0 on a root port, one more per hub above it.
pub fn set_hub_depth(xhci_port: u32, slot: u8, depth: u8) -> bool {
    let mut none = [0u8; 0];
    control_transfer(xhci_port, slot, RT_HUB_OUT, SET_HUB_DEPTH, depth as u16, 0, 0, &mut none)
        .is_ok()
}
