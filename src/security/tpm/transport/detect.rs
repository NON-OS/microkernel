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

use super::acpi::{control_area, start_method, START_ACPI, START_CRB, START_CRB_ACPI, START_FIFO};
use super::ident::{identify, Interface};
use crate::security::tpm::error::TpmError;
use crate::security::tpm::mmio::TPM_MMIO_BASE;

pub(super) fn detect() -> Result<Interface, TpmError> {
    let acpi = start_method();
    let wanted = match acpi {
        None => None,
        Some(START_FIFO) => Some(Interface::Fifo),
        Some(START_CRB | START_CRB_ACPI) => Some(Interface::Crb),
        Some(START_ACPI) => {
            crate::log::warn!(
                "[TPM] ACPI start method 2: the doorbell is an ACPI _DSM call this kernel cannot make"
            );
            return Err(TpmError::NotPresent);
        }
        Some(other) => {
            crate::log::warn!(
                "[TPM] ACPI start method {} is neither FIFO (6) nor CRB (7, 8)",
                other
            );
            return Err(TpmError::NotPresent);
        }
    };
    /*
     * A CRB control area outside the register window is a firmware TPM with
     * no window at all (AMD's): there is no identity register to ask, and the
     * table is the only word on what the part is.
     */
    if wanted == Some(Interface::Crb) {
        if let Some(at) = control_area().filter(|&at| !in_window(at)) {
            let line = alloc::format!("[TPM] interface CRB, control area at {:#X} (ACPI)", at);
            crate::sys::serial::println(line.as_bytes());
            crate::log::info!("{}", line);
            return Ok(Interface::Crb);
        }
    }
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
    announce(found, id, acpi.is_some());
    Ok(found)
}

/// The one bring-up line, on the serial console and in the log: the register
/// file, the part's interface id, and whether ACPI named it too.
fn announce(found: Interface, id: u32, acpi: bool) {
    let name = match found {
        Interface::Fifo => "FIFO (TIS)",
        Interface::Crb => "CRB",
    };
    let by = if acpi { "ACPI agrees" } else { "no ACPI start method" };
    let line = alloc::format!("[TPM] interface {} at {:#X}, locality 0 (id {:#X}, {})", name, TPM_MMIO_BASE, id, by);
    crate::sys::serial::println(line.as_bytes());
    crate::log::info!("{}", line);
}

/// Whether `at` is in the register window at 0xFED40000.
fn in_window(at: u64) -> bool {
    (TPM_MMIO_BASE..TPM_MMIO_BASE + 0x5000).contains(&at)
}
