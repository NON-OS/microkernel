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

//! HMAT read once at boot through the one decoder (`hmat_entry`), and kept
//! for the broker's NUMA queries, as SLIT's matrix is.

use super::super::state::TableRegistry;
use super::hmat_entry::{structures, Hmat};
use super::hmat_query::{query, HmatValue};
use crate::arch::x86_64::acpi::tables::{SdtHeader, MAX_TABLE_BYTES};
use spin::RwLock;

const SIG_HMAT: u32 = u32::from_le_bytes(*b"HMAT");

static HMAT: RwLock<Option<Hmat>> = RwLock::new(None);

pub fn parse_hmat(registry: &mut TableRegistry) {
    let Some(&phys) = registry.tables.get(&SIG_HMAT) else { return };
    let Some(addr) = super::super::phys::directmap(phys) else { return };
    // SAFETY: as for SRAT and SLIT: the directmap covers what firmware
    // published, and the length is trusted only once the checksum holds.
    let table = unsafe {
        let header = core::ptr::read_volatile(addr as *const SdtHeader);
        if !header.validate_checksum(addr as *const u8) {
            return;
        }
        let len = u64::from(header.length).min(MAX_TABLE_BYTES) as usize;
        core::slice::from_raw_parts(addr as *const u8, len)
    };
    *HMAT.write() = Some(structures(table));
}

/// A latency or bandwidth from initiator to target domain.
pub fn hmat_value(hierarchy: u8, data_type: u8, initiator: u32, target: u32) -> Option<HmatValue> {
    query(&HMAT.read().as_ref()?.matrices, hierarchy, data_type, initiator, target)
}

/// The initiator domain HMAT attaches a memory domain to.
#[allow(dead_code)] // the allocator's, when it reads NUMA facts
pub fn hmat_attached_initiator(memory: u32) -> Option<u32> {
    let h = HMAT.read();
    h.as_ref()?.attributes.iter().find(|a| a.memory == memory).and_then(|a| a.initiator)
}
