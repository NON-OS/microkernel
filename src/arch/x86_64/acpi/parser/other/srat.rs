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

//! SRAT read once at boot through the one pure decoder (`srat_entry`):
//! each processor's proximity domain and the memory ranges by domain. GICC
//! and Generic Initiator entries are decoded too, and kept for the device
//! records once their encoding is settled.

use super::super::state::TableRegistry;
use super::srat_entry::{entries, SratEntry};
use crate::arch::x86_64::acpi::data::NumaMemoryRegion;
use crate::arch::x86_64::acpi::tables::{SdtHeader, MAX_TABLE_BYTES, SIG_SRAT};

pub fn parse_srat(registry: &mut TableRegistry) {
    let Some(&phys) = registry.tables.get(&SIG_SRAT) else { return };
    let Some(addr) = super::super::phys::directmap(phys) else { return };
    // SAFETY: the registry holds tables firmware published and the directmap
    // covers them; the header is read before the length it carries is
    // trusted, and the checksum is checked over that length.
    let table = unsafe {
        let header = core::ptr::read_volatile(addr as *const SdtHeader);
        if !header.validate_checksum(addr as *const u8) {
            return;
        }
        let len = u64::from(header.length).min(MAX_TABLE_BYTES) as usize;
        core::slice::from_raw_parts(addr as *const u8, len)
    };
    for e in entries(table) {
        apply(registry, e);
    }
}

fn apply(registry: &mut TableRegistry, e: SratEntry) {
    match e {
        SratEntry::Apic { apic_id, domain } => {
            let found = registry.data.processors.iter_mut().find(|p| p.apic_id == apic_id);
            if let Some(p) = found {
                p.proximity_domain = domain;
            }
        }
        SratEntry::Memory { domain, base, len, hot_pluggable, non_volatile } => {
            registry.data.numa_regions.push(NumaMemoryRegion {
                base,
                length: len,
                proximity_domain: domain,
                hot_pluggable,
                non_volatile,
            });
        }
        SratEntry::Gicc { .. } | SratEntry::Initiator { .. } => {}
    }
}
