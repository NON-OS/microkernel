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

//! PCI_Config: the function the region's _ADR path reaches, then the
//! kernel's configuration access, refused for writes to a function another
//! capsule claimed.

use crate::sys::acpi_pci::{pci_read, pci_write};
use nonos_aml::exec::PciPath;
use nonos_aml::platform::{function_of, SECONDARY_BUS};

fn function(path: &PciPath) -> Option<(u16, u8, u8, u8)> {
    function_of(path, &mut |f| pci_read(f, SECONDARY_BUS, 1).ok().map(|v| v as u8))
}

/// Configuration accesses are 1, 2 or 4 bytes; an eight-byte one is two.
pub fn read(path: &PciPath, reg: u64, width: u32) -> Option<u64> {
    let f = function(path)?;
    let reg = u16::try_from(reg).ok()?;
    match width {
        1 | 2 | 4 => pci_read(f, reg, width as u8).ok().map(u64::from),
        8 => {
            let lo = pci_read(f, reg, 4).ok()? as u64;
            Some(lo | (pci_read(f, reg + 4, 4).ok()? as u64) << 32)
        }
        _ => None,
    }
}

pub fn write(path: &PciPath, reg: u64, width: u32, v: u64) -> bool {
    let (Some(f), Ok(reg)) = (function(path), u16::try_from(reg)) else { return false };
    match width {
        1 | 2 | 4 => pci_write(f, reg, width as u8, v as u32).is_ok(),
        8 => {
            pci_write(f, reg, 4, v as u32).is_ok()
                && pci_write(f, reg + 4, 4, (v >> 32) as u32).is_ok()
        }
        _ => false,
    }
}
