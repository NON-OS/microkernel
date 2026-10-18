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

//! A capsule's line under AMD interrupt remapping: an entry at index
//! `vector` in the table of the IO-APIC's own requester id (its IVHD
//! special entry), then the IO-APIC entry in compatibility format naming
//! that vector, so a level line's EOI still matches it. A line whose
//! IO-APIC IVRS does not name is refused. The entry is not handed back at
//! teardown: only the kernel writes IO-APIC entries, and the next line
//! with that vector rewrites it.

use super::super::error::{IoApicError, IoApicResult};
use super::super::ops_helpers::locate;
use super::super::types::Rte;
use super::program_route::write_route;
use crate::arch::x86_64::acpi::parser::other::amd_ioapic_requester;
use crate::arch::x86_64::iommu::amd_vi::intr;

pub(super) fn program_amd(gsi: u32, rte: Rte) -> IoApicResult<Option<u16>> {
    let (chip, _) = locate(gsi).ok_or(IoApicError::GsiNotFound)?;
    let all = crate::arch::x86_64::acpi::ioapics();
    let id = all.iter().find(|io| io.gsi_base == chip.gsi_base).ok_or(IoApicError::NotRemapped)?.id;
    let source = amd_ioapic_requester(id).ok_or(IoApicError::NotRemapped)?;
    let dest = crate::arch::x86_64::interrupt::apic::device_irq_dest(rte.dest_apic_id)
        .ok_or(IoApicError::NoReachableCpu)?;
    intr::route(source, rte.vector, dest as u8).map_err(|_| IoApicError::NotRemapped)?;
    write_route(gsi, rte).map(|()| None)
}
