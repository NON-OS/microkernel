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

//! Which register file is behind the window.
//!
//! The ACPI start method says what the firmware set up; TPM_INTERFACE_ID
//! says what the part presents. Both are asked, and a disagreement is
//! refused: the two register files overlap, so driving one through the
//! other's offsets writes command bytes into control registers.

use super::acpi::{start_method, START_CRB, START_CRB_ACPI, START_FIFO};
use super::ident::{identify, Interface};
use crate::security::tpm::error::TpmError;
use crate::security::tpm::mmio::TPM_MMIO_BASE;

pub(super) fn detect() -> Result<Interface, TpmError> {
    let acpi = start_method();
    let wanted = match acpi {
        None => None,
        Some(START_FIFO) => Some(Interface::Fifo),
        Some(START_CRB | START_CRB_ACPI) => Some(Interface::Crb),
        Some(other) => {
            crate::log::warn!(
                "[TPM] ACPI start method {} is neither FIFO (6) nor CRB (7, 8)",
                other
            );
            return Err(TpmError::NotPresent);
        }
    };
    let (id, found) = identify()?;
    let Some(found) = found else {
        crate::log::warn!("[TPM] no TPM at {:#X} (interface id {:#X})", TPM_MMIO_BASE, id);
        return Err(TpmError::NotPresent);
    };
    if let (Some(method), Some(want)) = (acpi, wanted) {
        if want != found {
            crate::log::warn!(
                "[TPM] ACPI start method {} disagrees with interface id {:#X}",
                method,
                id
            );
            return Err(TpmError::NotPresent);
        }
    }
    Ok(found)
}
