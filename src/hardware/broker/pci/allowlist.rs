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

//! Pure validator for `MkPciConfigWrite`. The whole authority lives here
//! and in `command`: only PCI Command bits 1 (Memory Space), 2 (Bus Master
//! Enable) and 10 (Interrupt Disable) and the MSI-X Message Control
//! register's Function Mask + Enable bits may flip; the caller
//! (`write::write`, through `ownership::resolve`) has already checked that
//! the writing pid holds the device's claim at the current epoch. Every
//! other config-space write (BARs, interrupt line, IDs, status, expansion
//! ROM, capability pointers, PCIe / AER) is refused before it reaches the
//! bus, but for the few vendor bits `quirk_bits` names. With a change mask
//! (`change`), each rule judges the register as the write would leave it.

use crate::drivers::pci::constants::{CFG_COMMAND, MSIX_CTRL_ENABLE, MSIX_CTRL_FUNCTION_MASK};
use crate::drivers::pci::types::MsixInfo;

use super::change::{merge, split};
use super::command::validate_command;
use super::quirk_bits::{only, writable, Ident};
use super::types::{PciWriteError, PciWriteRequest, WriteAction};

const MSIX_CONTROL_WRITABLE: u16 = MSIX_CTRL_ENABLE | MSIX_CTRL_FUNCTION_MASK;

pub fn validate(
    req: &PciWriteRequest,
    msix: Option<&MsixInfo>,
    ident: &Ident,
    current_register: u16,
) -> Result<WriteAction, PciWriteError> {
    let (register, change) = split(req.offset).ok_or(PciWriteError::OffsetNotAllowed)?;
    let value = match change {
        0 => req.value,
        m => merge(req.value, m, current_register).ok_or(PciWriteError::BitsNotAllowed)?,
    };
    if register == CFG_COMMAND {
        return validate_command(value, current_register, change != 0);
    }
    if let Some(m) = msix.filter(|m| register == m.offset as u16 + 2) {
        return validate_msix_control(value, current_register, m.offset as u16 + 2);
    }
    /* The vendor bits of `quirk_bits`, and nothing beside them. */
    if let Some(mask) = writable(ident, register as u32) {
        if !only(mask, value, current_register) {
            return Err(PciWriteError::BitsNotAllowed);
        }
        return Ok(WriteAction::Bits { offset: register, value });
    }
    Err(PciWriteError::OffsetNotAllowed)
}

fn validate_msix_control(
    new: u16,
    current: u16,
    offset: u16,
) -> Result<WriteAction, PciWriteError> {
    if (new ^ current) & !MSIX_CONTROL_WRITABLE != 0 {
        return Err(PciWriteError::BitsNotAllowed);
    }
    Ok(WriteAction::MsixControl { offset, value: new })
}
