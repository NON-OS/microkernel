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

//! PCI_Config regions: the kernel's own configuration path, reads of any
//! function, writes only to one no other capsule claimed.

use super::error::RegionError;
use super::pci_admit::admit;

pub fn pci_read(pid: u32, bdf: (u16, u8, u8, u8), at: u16, width: u8) -> Result<u32, RegionError> {
    let cfg = admit(pid, bdf, at, width, false)?;
    match width {
        1 => cfg.read8(at).map(u32::from),
        2 => cfg.read16(at).map(u32::from),
        _ => cfg.read32(at),
    }
    .map_err(|_| RegionError::Platform)
}

pub fn pci_write(
    pid: u32,
    bdf: (u16, u8, u8, u8),
    at: u16,
    width: u8,
    value: u32,
) -> Result<(), RegionError> {
    let cfg = admit(pid, bdf, at, width, true)?;
    match width {
        1 => cfg.write8(at, value as u8),
        2 => cfg.write16(at, value as u16),
        _ => cfg.write32(at, value),
    }
    .map_err(|_| RegionError::Platform)
}
