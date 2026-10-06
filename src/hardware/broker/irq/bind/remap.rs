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

//! Taking and giving back a VT-d interrupt remapping entry for a broker
//! vector, when the kernel is built with nonos-iommu-intremap.

use super::super::grant::NO_IRTE;
use super::message::Routed;
use crate::arch::x86_64::iommu::remap::{release_msi, route_msi};
use crate::arch::x86_64::iommu::types::VtdError;
use crate::drivers::pci::types::{MsiMessage, PciAddress};

/// `None` sends the compatibility format message, which every unit still
/// passes while CFI is set. A function behind a VMD (segment above 0) raises
/// its interrupts with the VMD's requester id, so an entry checked against
/// its own bus:dev.fn would block it.
pub(super) fn remapped(address: &PciAddress, vector: u8, dest: u8) -> Option<Routed> {
    if address.segment != 0 {
        return None;
    }
    let bdf = (address.bus, address.device, address.function);
    match route_msi(bdf, vector, dest as u32) {
        Ok((irte, m)) => {
            let msg = MsiMessage { address: m.address as u64, data: m.data };
            Some(Routed { msg, irte })
        }
        Err(VtdError::InterruptRemappingOff) => None,
        Err(e) => {
            crate::log::info!("[IRQ] {} vector {:#x} not remapped: {:?}", address, vector, e);
            None
        }
    }
}

pub(super) fn release(irte: u16) {
    if irte == NO_IRTE {
        return;
    }
    if let Err(e) = release_msi(irte) {
        crate::log::info!("[IRQ] remap entry {} not released: {:?}", irte, e);
    }
}
