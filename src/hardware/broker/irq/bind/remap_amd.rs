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

//! A broker MSI or MSI-X vector under AMD interrupt remapping (K4): an
//! entry in the device's own table at index `vector`, so the message is
//! the plain one naming the vector, and the IOMMU delivers it only as that
//! entry says. The broker's grant keeps the vector as its handle.

use super::super::grant::NO_IRTE;
use super::message::Routed;
use super::remap_rule::Attempt;
use crate::arch::x86_64::iommu::amd_vi::domain::device_id;
use crate::arch::x86_64::iommu::amd_vi::intr;
use crate::drivers::pci::types::{MsiMessage, PciAddress};

pub(super) fn attempt(address: &PciAddress, vector: u8, dest: u8) -> (Attempt, Option<Routed>) {
    if address.segment != 0 {
        // Behind VMD the requester id is VMD's own: not this device's.
        return (Attempt::Failed, None);
    }
    let id = device_id(address.bus, address.device, address.function);
    match intr::route(id, vector, dest) {
        Ok(()) => {
            let msg = MsiMessage::for_local_apic(vector, dest);
            (Attempt::Routed, Some(Routed { msg, irte: vector as u16 }))
        }
        Err(e) => {
            crate::log::info!("[IRQ] {} vector {:#x} not remapped: {:?}", address, vector, e);
            (Attempt::Failed, None)
        }
    }
}

pub(super) fn release(address: &PciAddress, irte: u16) {
    if irte == NO_IRTE || address.segment != 0 {
        return;
    }
    let id = device_id(address.bus, address.device, address.function);
    if let Err(e) = intr::release(id, irte as u8) {
        crate::log::info!("[IRQ] {} remap entry {} not released: {:?}", address, irte, e);
    }
}
