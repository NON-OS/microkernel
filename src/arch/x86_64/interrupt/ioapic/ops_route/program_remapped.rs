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

//! A capsule's line under interrupt remapping (K3): a remapping entry for
//! the IO-APIC's own requester id, and the IO-APIC entry in remappable
//! format naming it, written masked. A compatibility format entry would be
//! blocked with CFI clear.

use super::super::error::{IoApicError, IoApicResult};
use super::super::mmio::{redtbl_read, redtbl_write};
use super::super::ops_helpers::locate;
use super::super::types::Rte;
use crate::arch::x86_64::iommu::remap::{handle_of, release_msi, remappable, route_entry, Route};
use crate::memory::proof::{self, CapTag};

/// Returns the remapping entry; the broker's grant gives it back.
pub(super) fn program_remapped(gsi: u32, rte: Rte) -> IoApicResult<u16> {
    let (chip, idx) = locate(gsi).ok_or(IoApicError::GsiNotFound)?;
    let source = source_of(chip.gsi_base).ok_or(IoApicError::NotRemapped)?;
    let destination = crate::arch::x86_64::interrupt::apic::device_irq_dest(rte.dest_apic_id)
        .ok_or(IoApicError::NoReachableCpu)?;
    let route = Route { vector: rte.vector, destination, source, level: rte.level_trigger };
    let handle = route_entry(route).map_err(|_| IoApicError::NotRemapped)?;
    let (low, high) = remappable(handle, rte.vector, rte.level_trigger, rte.active_low);
    // SAFETY: eK@nonos.systems - the chip's registers were mapped at init and
    // `idx` lies within its redirections; the entry is written masked.
    let (back_low, back_high) = unsafe {
        redtbl_write(chip.mmio, idx, low, high);
        redtbl_read(chip.mmio, idx)
    };
    // An IO-APIC that drops the format bit would send compatibility format.
    if handle_of(back_low, back_high) != Some(handle) {
        let _ = release_msi(handle);
        return Err(IoApicError::NotRemapped);
    }
    proof::audit_phys_alloc(
        ((gsi as u64) << 32) | rte.vector as u64,
        handle as u64,
        CapTag::KERNEL,
    );
    Ok(handle)
}

/// The requester id of the IO-APIC serving GSIs from `gsi_base`, from the
/// DRHD scope naming its MADT id.
fn source_of(gsi_base: u32) -> Option<u16> {
    use crate::arch::x86_64::acpi::parser::other::ioapic_source;
    use crate::drivers::pci::config::read8;
    use crate::drivers::pci::constants::CFG_SECONDARY_BUS;
    let all = crate::arch::x86_64::acpi::ioapics();
    let id = all.iter().find(|io| io.gsi_base == gsi_base)?.id;
    ioapic_source(id, |b, d, f| read8(b, d, f, CFG_SECONDARY_BUS).ok())
}
