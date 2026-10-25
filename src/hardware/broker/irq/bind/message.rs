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
//! remapping on, the message names an entry the IOMMU wrote for this
//! function (VT-d 3.4, 5.1.5.2; on AMD-Vi, the device's own table), or the
//! bind is refused. A compatibility format message naming a LAPIC directly
//! is made only with remapping off, under the development opt-in or the
//! no-IOMMU posture (K3, remap_rule.rs).

use super::super::types::IrqBindError;
use super::remap_rule::{outcome, Outcome};
use super::sentinels::NO_IRTE;
use crate::drivers::pci::types::{MsiMessage, PciAddress};
use crate::memory::iommu::dev_unremapped::holds;

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

pub(super) fn route(address: &PciAddress, vector: u8, dest: u8) -> Result<Routed, IrqBindError> {
    let (attempt, routed) = super::remap_vendor::attempt(address, vector, dest);
    match outcome(attempt, holds() || crate::memory::iommu::no_iommu::holds()) {
        Outcome::Remapped => routed.ok_or(IrqBindError::PlatformError),
        Outcome::Compat => {
            Ok(Routed { msg: MsiMessage::for_local_apic(vector, dest), irte: NO_IRTE })
        }
        Outcome::Full => Err(IrqBindError::NoVector),
        Outcome::Refused => {
            crate::log::info!("[IRQ] vector {:#x} refused: interrupt not remapped", vector);
            Err(IrqBindError::NotRemapped)
        }
    }
}

/// Gives a remapping entry back once the device can no longer raise it: an
/// AMD-Vi handle (bit 15) or a VT-d entry, by the handle alone.
pub(in super::super) fn release(irte: u16) {
    #[cfg(feature = "nonos-iommu-amdvi")]
    if super::amd_remap::is_amd(irte) {
        return super::amd_remap::release(irte);
    }
    #[cfg(feature = "nonos-iommu-intremap")]
    super::remap::release(irte);
    let _ = irte;
}
