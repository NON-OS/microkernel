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

//! Writing the bind's messages into the function: its MSI capability, or
//! the first entries of its MSI-X table with every other entry masked.

use crate::drivers::pci::config::ConfigSpace;
use crate::drivers::pci::msi::{
    enable_msix, mask_all_msix, mask_msix_vector, unmask_all_msix, unmask_msix_vector,
    write_msix_message,
};
use crate::drivers::pci::types::{MsiMessage, MsixInfo};
use crate::drivers::pci::PciError;
use crate::hardware::broker::irq::types::IrqBindError;
use crate::hardware::broker::pci_index::{msi_of, PciHandle};

use super::msi_program;
use super::record::MessageBind;

/// Point every vector at the doorbell, at `address` as the device sees it.
pub(super) fn program(
    b: &MessageBind,
    handle: &PciHandle,
    address: u64,
) -> Result<(), IrqBindError> {
    let messages = b.vectors[..b.count].iter().map(|v| MsiMessage { address, data: v.data });
    if b.msix {
        let msix = handle.msix.ok_or(IrqBindError::NoMsixCap)?;
        run(handle, &msix, b.count as u16, messages).map_err(|_| IrqBindError::MsixProgramFailed)
    } else {
        let msi = msi_of(handle).ok_or(IrqBindError::NoMsiCap)?;
        // A 32-bit capability keeps the low address word alone, and would
        // send the message somewhere else entirely.
        if !msi.is_64bit && address > u64::from(u32::MAX) {
            return Err(IrqBindError::MsiProgramFailed);
        }
        let first = messages.into_iter().next().ok_or(IrqBindError::BadVectorCount)?;
        msi_program::program(&handle.address, &msi, first)
            .map_err(|_| IrqBindError::MsiProgramFailed)
    }
}

/// The function masked while its table is written, as on x86, and each entry
/// past the bind masked first: firmware may have left one open and aimed.
fn run(
    handle: &PciHandle,
    msix: &MsixInfo,
    count: u16,
    messages: impl Iterator<Item = MsiMessage>,
) -> Result<(), PciError> {
    let cfg = ConfigSpace::new(handle.address);
    mask_all_msix(&cfg, msix)?;
    enable_msix(&cfg, msix)?;
    for rest in count..msix.vector_count() {
        mask_msix_vector(msix, &handle.bars, rest)?;
    }
    for (i, msg) in messages.enumerate() {
        write_msix_message(msix, &handle.bars, i as u16, msg)?;
        unmask_msix_vector(msix, &handle.bars, i as u16)?;
    }
    unmask_all_msix(&cfg, msix)
}
