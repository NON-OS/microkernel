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

//! Whether an access names a register at all: a device and function in range,
//! a width config space has, aligned to itself, inside the 4 KiB a PCIe
//! function decodes. Whether the register can be reached is the route's
//! question, not this one's. Pure, so the host proofs hold it.

use super::super::constants::{PCIE_CONFIG_SPACE_SIZE, PCI_MAX_DEVICE, PCI_MAX_FUNCTION};
use super::super::error::PciError;
use super::super::types::PciAddress;

pub const fn check(a: PciAddress, offset: u16, width: u8) -> Result<(), PciError> {
    if a.device > PCI_MAX_DEVICE {
        return Err(PciError::InvalidDevice(a.device));
    }
    if a.function > PCI_MAX_FUNCTION {
        return Err(PciError::InvalidFunction(a.function));
    }
    // Width first: the alignment mask below would wrap on a width of zero.
    if !matches!(width, 1 | 2 | 4) || offset & (width as u16 - 1) != 0 {
        return Err(PciError::UnalignedAccess { offset, alignment: width });
    }
    // Aligned and below the limit means the last byte is below it too.
    if offset >= PCIE_CONFIG_SPACE_SIZE {
        return Err(PciError::InvalidOffset(offset));
    }
    Ok(())
}
