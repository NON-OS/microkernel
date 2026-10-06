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
//! for this function (VT-d 3.4, 5.1.5.2); otherwise it is the compatibility
//! format message naming a LAPIC directly.

use super::super::grant::NO_IRTE;
use super::super::types::IrqBindError;
use crate::drivers::pci::types::{MsiMessage, PciAddress};

/// A message to program and the remapping entry behind it, `NO_IRTE` for a
/// compatibility format message.
#[derive(Clone, Copy)]
pub(super) struct Routed {
    pub(super) msg: MsiMessage,
    pub(super) irte: u16,
}

/// The LAPIC the message targets: the CPU binding it, or one that fits the
/// 8-bit destination field when its id does not.
pub(super) fn dest_apic_id() -> Result<u8, IrqBindError> {
    let local = crate::arch::interrupt_controller::local_id();
    crate::arch::x86_64::interrupt::apic::device_irq_dest(local)
        .map(|id| id as u8)
        .ok_or(IrqBindError::PlatformError)
}

pub(super) fn route(address: &PciAddress, vector: u8, dest: u8) -> Routed {
    #[cfg(feature = "nonos-iommu-intremap")]
    if let Some(r) = super::remap::remapped(address, vector, dest) {
        return r;
    }
    let _ = address;
    Routed { msg: MsiMessage::for_local_apic(vector, dest), irte: NO_IRTE }
}

/// Gives a remapping entry back once the device can no longer raise it.
pub(in super::super) fn release(irte: u16) {
    #[cfg(feature = "nonos-iommu-intremap")]
    super::remap::release(irte);
    let _ = irte;
}
