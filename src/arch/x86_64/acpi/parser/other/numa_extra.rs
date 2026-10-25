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

//! The NUMA facts beyond the processors' domains and the memory ranges:
//! SLIT's distances and SRAT's Generic Initiators, the devices that start
//! memory accesses from a domain of their own. Kept here, as IVRS's units
//! are, for the broker's device queries.

use super::super::state::TableRegistry;
use super::slit_matrix::matrix;
use super::srat_entry::Handle;
use crate::arch::x86_64::acpi::tables::{SdtHeader, MAX_TABLE_BYTES, SIG_SLIT};
use alloc::vec::Vec;
use spin::RwLock;

static DISTANCES: RwLock<Option<(usize, Vec<u8>)>> = RwLock::new(None);
static INITIATORS: RwLock<Vec<(u32, Handle)>> = RwLock::new(Vec::new());

pub(super) fn add_initiator(domain: u32, handle: Handle) {
    INITIATORS.write().push((domain, handle));
}

pub fn parse_slit(registry: &mut TableRegistry) {
    let Some(&phys) = registry.tables.get(&SIG_SLIT) else { return };
    let Some(addr) = super::super::phys::directmap(phys) else { return };
    // SAFETY: as for SRAT: the directmap covers what firmware published,
    // and the length is trusted only once the checksum holds over it.
    let table = unsafe {
        let header = core::ptr::read_volatile(addr as *const SdtHeader);
        if !header.validate_checksum(addr as *const u8) {
            return;
        }
        let len = u64::from(header.length).min(MAX_TABLE_BYTES) as usize;
        core::slice::from_raw_parts(addr as *const u8, len)
    };
    *DISTANCES.write() = matrix(table);
}

/// SLIT's distance between two localities (proximity domains).
pub fn numa_distance(from: u32, to: u32) -> Option<u8> {
    let d = DISTANCES.read();
    let (n, m) = d.as_ref()?;
    let (from, to) = (usize::try_from(from).ok()?, usize::try_from(to).ok()?);
    (from < *n && to < *n).then(|| m[from * n + to])
}

pub fn numa_initiators() -> Vec<(u32, Handle)> {
    INITIATORS.read().clone()
}
