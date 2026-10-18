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

//! A table's bytes by signature, for the parsers that check the header,
//! length and checksum themselves (the ones shared with aarch64).

use super::super::state::TableRegistry;
use crate::arch::x86_64::acpi::tables::{SdtHeader, MAX_TABLE_BYTES};

pub(super) fn bytes(registry: &TableRegistry, sig: &[u8; 4]) -> Option<&'static [u8]> {
    let phys = *registry.tables.get(&u32::from_le_bytes(*sig))?;
    let addr = super::super::phys::directmap(phys)?;
    // SAFETY: the directmap covers what firmware published; the parsers
    // check the length against these bytes and the checksum themselves.
    unsafe {
        let len = core::ptr::read_volatile(addr as *const SdtHeader).length;
        let len = u64::from(len).min(MAX_TABLE_BYTES) as usize;
        Some(core::slice::from_raw_parts(addr as *const u8, len))
    }
}
