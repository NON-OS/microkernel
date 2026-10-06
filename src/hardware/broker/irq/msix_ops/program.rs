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

#![cfg(target_arch = "x86_64")]

//! Programming a run of MSI-X table entries with the messages the bind
//! composed, one per entry from entry 0: the function masked while the
//! entries are written, each entry unmasked, then the function unmasked.

use crate::drivers::pci::config::ConfigSpace;
use crate::drivers::pci::msi::{
    enable_msix, mask_all_msix, unmask_all_msix, unmask_msix_vector, write_msix_message,
};
use crate::drivers::pci::types::{MsiMessage, MsixInfo, PciAddress, PciBar};

use super::super::types::IrqBindError;

pub(super) fn program_run(
    address: &PciAddress,
    msix: &MsixInfo,
    bars: &[PciBar; 6],
    messages: &[MsiMessage],
) -> Result<(), IrqBindError> {
    let cfg = ConfigSpace::new(*address);
    mask_all_msix(&cfg, msix).map_err(|_| IrqBindError::MsixProgramFailed)?;
    enable_msix(&cfg, msix).map_err(|_| IrqBindError::MsixProgramFailed)?;
    for (i, msg) in messages.iter().enumerate() {
        let vector = i as u16;
        write_msix_message(msix, bars, vector, *msg)
            .map_err(|_| IrqBindError::MsixProgramFailed)?;
        unmask_msix_vector(msix, bars, vector).map_err(|_| IrqBindError::MsixProgramFailed)?;
    }
    unmask_all_msix(&cfg, msix).map_err(|_| IrqBindError::MsixProgramFailed)
}
