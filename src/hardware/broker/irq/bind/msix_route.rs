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

//! Pointing a run of MSI-X entries at consecutive broker vectors: one
//! message per entry (remapped when interrupt remapping is on), then the
//! run programmed. Every remapping entry is given back if the table refused.

extern crate alloc;

use alloc::vec::Vec;

use super::super::msix_ops::current_ops;
use super::super::types::IrqBindError;
use super::message::{dest_apic_id, release, route, Routed};
use crate::drivers::pci::types::{MsiMessage, MsixInfo};
use crate::hardware::broker::pci_index::PciHandle;

/// The remapping entry behind each table entry, in table order.
pub(super) fn program_routed(
    handle: &PciHandle,
    msix: &MsixInfo,
    base_vector: u8,
    n: usize,
) -> Result<Vec<u16>, IrqBindError> {
    let dest = dest_apic_id()?;
    let routed: Vec<Routed> =
        (0..n).map(|i| route(&handle.address, base_vector + i as u8, dest)).collect();
    let messages: Vec<MsiMessage> = routed.iter().map(|r| r.msg).collect();
    if let Err(e) = current_ops().program_run(&handle.address, msix, &handle.bars, &messages) {
        routed.iter().for_each(|r| release(r.irte));
        return Err(e);
    }
    Ok(routed.iter().map(|r| r.irte).collect())
}
