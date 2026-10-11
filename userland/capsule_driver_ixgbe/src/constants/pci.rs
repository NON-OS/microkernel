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

//! The functions this driver takes: the 82599, X540 and X550 ids of Linux's
//! ixgbe_pci_tbl, each with its MAC. Backplane, CX4 and SFP+ 82599 ports are
//! listed as Linux lists them; their link is the mode firmware left in
//! AUTOC (init/autoc.rs), the SFP module setup Linux runs is not carried.

use super::part::Part;

pub const INTEL_VENDOR_ID: u16 = 0x8086;

pub const IXGBE_PARTS: &[(u16, Part)] = &[
    (0x10F7, Part::I82599), // 82599 KX4
    (0x1514, Part::I82599), // 82599 KX4 mezzanine
    (0x1517, Part::I82599), // 82599 KR
    (0x10F8, Part::I82599), // 82599 combo backplane
    (0x10F9, Part::I82599), // 82599 CX4
    (0x10FB, Part::I82599), // 82599 SFP+ (X520)
    (0x1529, Part::I82599), // 82599 SFP+ FCoE
    (0x152A, Part::I82599), // 82599 backplane FCoE
    (0x1557, Part::I82599), // 82599EN SFP+
    (0x10FC, Part::I82599), // 82599 XAUI LOM
    (0x151C, Part::I82599), // 82599 10GBASE-T LOM
    (0x154D, Part::I82599), // 82599 SFP+ SF2
    (0x1558, Part::I82599), // 82599 QSFP+
    (0x1528, Part::X540),   // X540-T2
    (0x1560, Part::X540),   // X540-T1
    (0x1563, Part::X550),   // X550-T2
    (0x15D1, Part::X550),   // X550-T1
];

pub fn part_of(device: u16) -> Option<Part> {
    IXGBE_PARTS.iter().find(|(id, _)| *id == device).map(|(_, part)| *part)
}

/// ixgbe_probe maps pci_resource_start(pdev, 0): the register file is BAR0.
pub const BAR_INDEX: u32 = 0;
pub const BAR_OFFSET: u64 = 0;
