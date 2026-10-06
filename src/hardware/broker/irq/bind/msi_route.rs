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

//! Pointing a function's MSI at the broker vector of one slot: the message
//! composed (remapped when interrupt remapping is on), then programmed. A
//! remapping entry taken for a message the function refused is given back.

use super::super::types::IrqBindError;
use super::message::{dest_apic_id, release, route, Routed};
use super::msi_program::program;
use crate::arch::interrupt::broker::vector_of;
use crate::drivers::pci::types::{MsiInfo, PciAddress};

/// The vector and remapping entry the function now raises.
pub(super) fn program_slot(
    slot: usize,
    address: &PciAddress,
    msi: &MsiInfo,
) -> Result<(u8, u16), IrqBindError> {
    let vector = vector_of(slot).ok_or(IrqBindError::NoVector)?;
    let Routed { msg, irte } = route(address, vector, dest_apic_id()?);
    if program(address, msi, msg).is_err() {
        release(irte);
        return Err(IrqBindError::MsiProgramFailed);
    }
    Ok((vector, irte))
}
