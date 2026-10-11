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
//! The parts this driver binds: every entry of igb_pci_tbl in Linux
//! igb_main.c, in its order, each with the MAC igb_set_mac_type gives it.
//! Fiber, SerDes, SGMII and backplane parts are listed too, marked Other:
//! discovery takes a copper port first, and bring-up refuses the others.

use super::part::Media::{Copper, Other};
use super::part::{Media, Part};

pub const INTEL_VENDOR_ID: u16 = 0x8086;

pub const IGB_PARTS: &[(u16, Part, Media)] = &[
    (0x1F40, Part::I354, Other),    // I354 backplane 1 Gb/s
    (0x1F41, Part::I354, Other),    // I354 SGMII
    (0x1F45, Part::I354, Other),    // I354 backplane 2.5 Gb/s
    (0x1539, Part::I211, Copper),   // I211 copper
    (0x1533, Part::I210, Copper),   // I210 copper
    (0x1536, Part::I210, Other),    // I210 fiber
    (0x1537, Part::I210, Other),    // I210 SerDes
    (0x1538, Part::I210, Other),    // I210 SGMII
    (0x157B, Part::I210, Copper),   // I210 copper, flashless
    (0x157C, Part::I210, Other),    // I210 SerDes, flashless
    (0x1521, Part::I350, Copper),   // I350 copper
    (0x1522, Part::I350, Other),    // I350 fiber
    (0x1523, Part::I350, Other),    // I350 SerDes
    (0x1524, Part::I350, Other),    // I350 SGMII
    (0x150E, Part::I82580, Copper), // 82580 copper
    (0x150F, Part::I82580, Other),  // 82580 fiber
    (0x1527, Part::I82580, Other),  // 82580 quad fiber
    (0x1510, Part::I82580, Other),  // 82580 SerDes
    (0x1511, Part::I82580, Other),  // 82580 SGMII
    (0x1516, Part::I82580, Copper), // 82580 copper, dual
    (0x0438, Part::I82580, Other),  // DH89xxCC SGMII
    (0x043A, Part::I82580, Other),  // DH89xxCC SerDes
    (0x043C, Part::I82580, Other),  // DH89xxCC backplane
    (0x0440, Part::I82580, Other),  // DH89xxCC SFP
    (0x10C9, Part::I82576, Copper), // 82576, QEMU's igb
    (0x150A, Part::I82576, Copper), // 82576 NS
    (0x1518, Part::I82576, Other),  // 82576 NS SerDes
    (0x10E6, Part::I82576, Other),  // 82576 fiber
    (0x10E7, Part::I82576, Other),  // 82576 SerDes
    (0x150D, Part::I82576, Other),  // 82576 SerDes quad
    (0x1526, Part::I82576, Copper), // 82576 quad copper ET2
    (0x10E8, Part::I82576, Copper), // 82576 quad copper
    (0x10A7, Part::I82575, Copper), // 82575EB copper
    (0x10A9, Part::I82575, Other),  // 82575EB fiber/SerDes
    (0x10D6, Part::I82575, Copper), // 82575GB quad copper
];

pub fn part_of(device: u16) -> Option<Part> {
    IGB_PARTS.iter().find(|(id, ..)| *id == device).map(|(_, part, _)| *part)
}

/// Whether the function is a copper port, the only kind brought up.
pub fn is_copper(device: u16) -> bool {
    IGB_PARTS.iter().any(|&(id, _, m)| id == device && m == Copper)
}

/// igb_probe maps pci_resource_start(pdev, 0): the register file is BAR0.
pub const BAR_INDEX: u32 = 0;
pub const BAR_OFFSET: u64 = 0;
