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

//! What a bridge's secondary bus is: PCI Express, or conventional PCI
//! behind a PCIe-to-PCI/PCI-X bridge (device/port type 7) or a bridge with
//! no PCI Express capability. Devices on a conventional bus reach each
//! other without any port, and reach the IOMMU under the bridge's alias.

use super::acs_tree::Bdf;
use crate::drivers::pci::config::read32_unchecked;

const STATUS_CAP_LIST: u32 = 1 << 20;
const PCIE_CAP_ID: u32 = 0x10;
const PCIE_TO_PCI: u32 = 0x7;

#[derive(Clone, Copy, PartialEq, Eq)]
pub(super) enum Secondary {
    Express,
    PcieToPci,
    LegacyPci,
}

pub(super) fn secondary(at: Bdf) -> Secondary {
    match pcie_head(at) {
        None => Secondary::LegacyPci,
        Some(head) if (head >> 20) & 0xF == PCIE_TO_PCI => Secondary::PcieToPci,
        Some(_) => Secondary::Express,
    }
}

/// The device/port type of its PCI Express capability, None without one.
pub(super) fn pcie_type(at: Bdf) -> Option<u8> {
    pcie_head(at).map(|head| ((head >> 20) & 0xF) as u8)
}

/// The first dword of the PCI Express capability, walking at most 48.
fn pcie_head((b, d, f): Bdf) -> Option<u32> {
    if read32_unchecked(b, d, f, 0x04) & STATUS_CAP_LIST == 0 {
        return None;
    }
    let mut at = (read32_unchecked(b, d, f, 0x34) & 0xFC) as u8;
    for _ in 0..48 {
        if at < 0x40 {
            return None;
        }
        let head = read32_unchecked(b, d, f, at);
        if head & 0xFF == PCIE_CAP_ID {
            return Some(head);
        }
        at = ((head >> 8) & 0xFC) as u8;
    }
    None
}
