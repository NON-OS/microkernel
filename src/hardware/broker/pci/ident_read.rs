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

//! What the vendor-bit rules (quirk_bits) are told of the function being
//! written, read from its own config space at the time of the write.

use crate::drivers::pci::config::ConfigSpace;

use super::quirk_bits::{legsup_before, Ident, Route};
use super::types::PciWriteError;

/// The function's vendor, class and PCI Express capability, read from its
/// own config space at the time of the write.
pub(super) fn ident(cfg: &ConfigSpace, offset: u32) -> Result<Ident, PciWriteError> {
    let vendor = cfg.read16(0x00).map_err(|_| PciWriteError::PlatformError)?;
    let device = cfg.read16(0x02).map_err(|_| PciWriteError::PlatformError)?;
    let class = cfg.read32(0x08).map_err(|_| PciWriteError::PlatformError)?;
    /*
     * The capability walk reads segment 0. A function behind a VMD lives in
     * a domain of its own, so the same bus, device and function there name
     * another device: none of its capability bits are offered.
     */
    let at = cfg.address();
    let pcie_cap = (at.segment == 0)
        .then(|| {
            crate::drivers::pci::capabilities::find_capability(
                at.bus,
                at.device,
                at.function,
                crate::drivers::pci::constants::CAP_ID_PCIE,
            )
        })
        .flatten()
        .map(|c| c.offset as u16);
    let (subclass, prog_if) = ((class >> 16) as u8, (class >> 8) as u8);
    let ehci = class >> 8 == 0x0C_0320;
    let read = |o: u16| cfg.read8(o).unwrap_or(0xFF);
    let before = if ehci { legsup_before(&read, offset) } else { [0xFF; 3] };
    let route = if vendor == 0x8086 && class >> 8 == 0x0C_0330 {
        // The masks as firmware has them now, and whether EHCI is held now.
        let mask = |o: u16| cfg.read16(o).unwrap_or(0);
        let ehci_claimed = super::companion::ehci_claimed(at);
        Route { usb2prm: mask(0xD4), usb3prm: mask(0xDC), ehci_claimed }
    } else {
        Route::NONE
    };
    let class_byte = (class >> 24) as u8;
    Ok(Ident { vendor, device, class: class_byte, subclass, prog_if, pcie_cap, before, route })
}
