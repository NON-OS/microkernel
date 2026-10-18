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

//! LPIT read once at boot through the one decoder (`lpit_entry`) and kept
//! for the ACPI capsule's suspend-to-idle; `lpit_counter` reads each
//! state's residency counter on request.

use super::super::state::TableRegistry;
use super::lpit_entry::{states, LpiState};
use crate::arch::x86_64::acpi::tables::{SdtHeader, MAX_TABLE_BYTES};
use alloc::vec::Vec;
use spin::RwLock;

const SIG_LPIT: u32 = u32::from_le_bytes(*b"LPIT");

static LPIT: RwLock<Vec<LpiState>> = RwLock::new(Vec::new());

pub fn parse_lpit(registry: &mut TableRegistry) {
    let Some(&phys) = registry.tables.get(&SIG_LPIT) else { return };
    let Some(addr) = super::super::phys::directmap(phys) else { return };
    // SAFETY: as for SRAT and HMAT: the directmap covers what firmware
    // published, and the length is trusted only once the checksum holds.
    let table = unsafe {
        let header = core::ptr::read_volatile(addr as *const SdtHeader);
        if !header.validate_checksum(addr as *const u8) {
            return;
        }
        let len = u64::from(header.length).min(MAX_TABLE_BYTES) as usize;
        core::slice::from_raw_parts(addr as *const u8, len)
    };
    *LPIT.write() = states(table);
}

/// The `index`th low-power idle state.
pub fn lpit_state(index: usize) -> Option<LpiState> {
    LPIT.read().get(index).copied()
}
