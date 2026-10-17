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

pub(super) fn secondary((b, d, f): Bdf) -> Secondary {
    if read32_unchecked(b, d, f, 0x04) & STATUS_CAP_LIST == 0 {
        return Secondary::LegacyPci;
    }
    let mut at = (read32_unchecked(b, d, f, 0x34) & 0xFC) as u8;
    for _ in 0..48 {
        if at < 0x40 {
            return Secondary::LegacyPci;
        }
        let head = read32_unchecked(b, d, f, at);
        if head & 0xFF == PCIE_CAP_ID {
            let to_pci = (head >> 20) & 0xF == PCIE_TO_PCI;
            return if to_pci { Secondary::PcieToPci } else { Secondary::Express };
        }
        at = ((head >> 8) & 0xFC) as u8;
    }
    Secondary::LegacyPci
}
