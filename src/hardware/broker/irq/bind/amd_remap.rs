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

//! Taking and giving back an AMD-Vi remapped entry for a broker vector, when
//! the kernel is built with nonos-iommu-amdvi (L42).

use super::message::Routed;
use crate::arch::x86_64::iommu::amd_vi::{intr, AmdViError};
use crate::drivers::pci::types::{MsiMessage, PciAddress};

/// `None` sends the compatibility format message: on a machine with no
/// AMD unit enforcing, and for a function behind a VMD, whose interrupts
/// carry the VMD's requester id. Any other refusal is logged: a device
/// already remapped then raises nothing for that vector, which is safer
/// than raising anything it likes.
pub(super) fn remapped(address: &PciAddress, vector: u8, dest: u8) -> Option<Routed> {
    if address.segment != 0 {
        return None;
    }
    let bdf = (address.bus, address.device, address.function);
    match intr::route_msi(bdf, vector, dest) {
        Ok((irte, address, data)) => Some(Routed { msg: MsiMessage { address, data }, irte }),
        Err(AmdViError::NotEnforcing | AmdViError::NotPresent) => None,
        Err(e) => {
            crate::log::info!("[IRQ] {} vector {:#x} not remapped: {:?}", address, vector, e);
            None
        }
    }
}

/// Whether a broker handle is one of AMD-Vi's (bit 15 set).
pub(super) fn is_amd(irte: u16) -> bool {
    intr::is_amd(irte)
}

pub(super) fn release(irte: u16) {
    if let Err(e) = intr::release_msi(irte) {
        crate::log::info!("[IRQ] remap handle {:#x} not released: {:?}", irte, e);
    }
}
