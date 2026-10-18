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

//! The one place a broker MSI or MSI-X message is composed. With interrupt
//! remapping on, the message names an IRTE the IOMMU's route_msi wrote
//! for this function (VT-d 3.4, 5.1.5.2), or the bind is refused. A
//! compatibility format message naming a LAPIC directly is made only with
//! remapping off and under the development opt-in (remap_rule).

use super::super::grant::NO_IRTE;
use super::super::types::IrqBindError;
use super::remap_rule::{outcome, Outcome};
use crate::drivers::pci::types::{MsiMessage, PciAddress};
use crate::memory::iommu::dev_unremapped::holds;

#[derive(Clone, Copy)]
pub(super) struct Routed {
    pub(super) msg: MsiMessage,
    pub(super) irte: u16,
}

pub(super) fn dest_apic_id() -> Result<u8, IrqBindError> {
    let local = crate::arch::interrupt_controller::local_id();
    crate::arch::x86_64::interrupt::apic::device_irq_dest(local)
        .map(|id| id as u8)
        .ok_or(IrqBindError::PlatformError)
}

pub(super) fn route(address: &PciAddress, vector: u8, dest: u8) -> Result<Routed, IrqBindError> {
    let (attempt, routed) = super::remap_vendor::attempt(address, vector, dest);
    match outcome(attempt, holds()) {
        Outcome::Remapped => routed.ok_or(IrqBindError::PlatformError),
        Outcome::Compat => {
            Ok(Routed { msg: MsiMessage::for_local_apic(vector, dest), irte: NO_IRTE })
        }
        Outcome::Full => Err(IrqBindError::NoVector),
        Outcome::Refused => {
            crate::log::info!("[IRQ] {} refused: interrupt not remapped", address);
            Err(IrqBindError::NotRemapped)
        }
    }
}

pub(super) fn release(address: &PciAddress, irte: u16) {
    super::remap_vendor::release(address, irte);
}

/// For the teardown, which knows the grant's device by its broker id.
pub(in super::super) fn release_for(device_id: u64, irte: u16) {
    if let Some(handle) = crate::hardware::broker::pci_index::lookup(device_id) {
        release(&handle.address, irte);
    }
}
