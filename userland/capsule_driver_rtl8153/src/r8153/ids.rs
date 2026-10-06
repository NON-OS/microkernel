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

//! The adapters r8152.c's rtl8152_table lists that this driver takes. The
//! Realtek product IDs that name another chip are left out: 0x8152 (the
//! 100 Mb/s RTL8152), 0x8155 and 0x8156 (the multi-gigabit RTL8156
//! family), and 0x8050 and 0x8053, whose chip r8152.c does not name. The
//! D-Link DUB-E250 (2001:b301), sold as a 2.5 Gb/s adapter, is left out
//! too. The OEM adapters below do not say their chip in their IDs; for
//! them the version register decides (version.rs), and one built on
//! another chip is refused there by name.

const RTL8153_FAMILY: &[(u16, u16)] = &[
    // Realtek RTL8153 and RTL8153B adapters.
    (0x0bda, 0x8153),
    // Microsoft Surface Ethernet adapters and the Surface Dock.
    (0x045e, 0x07ab),
    (0x045e, 0x07c6),
    (0x045e, 0x0927),
    (0x045e, 0x0c5e),
    // Samsung USB Ethernet adapter.
    (0x04e8, 0xa101),
    // Lenovo and ThinkPad docks, dongles and the USB-C Travel Hub.
    (0x17ef, 0x304f),
    (0x17ef, 0x3054),
    (0x17ef, 0x3062),
    (0x17ef, 0x3069),
    (0x17ef, 0x3082),
    (0x17ef, 0x7205),
    (0x17ef, 0x720c),
    (0x17ef, 0x7214),
    (0x17ef, 0x721e),
    (0x17ef, 0xa387),
    // Linksys USB3GIGV1, NVIDIA Shield, TP-Link UE300.
    (0x13b1, 0x0041),
    (0x0955, 0x09ff),
    (0x2357, 0x0601),
];

pub fn is_rtl8153_adapter(vendor: u16, product: u16) -> bool {
    RTL8153_FAMILY.contains(&(vendor, product))
}
