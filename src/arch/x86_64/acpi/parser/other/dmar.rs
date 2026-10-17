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

use core::mem;
use core::ptr;

use super::super::state::TableRegistry;
use super::dmar_entry::entry;
use crate::arch::x86_64::acpi::tables::{Dmar, SIG_DMAR};

pub fn parse_dmar(registry: &mut TableRegistry) {
    let addr = match registry.tables.get(&SIG_DMAR) {
        Some(&a) => a,
        None => return,
    };
    let Some((addr, header)) = super::super::phys::mapped_table(addr) else {
        return;
    };
    let mut found_drhd = false;
    // SAFETY: eK@nonos.systems - `addr` is the directmapped table ACPI
    // published, mapped for its whole length (mapped_table) and read whole by
    // its checksum; every structure read below lies within `[addr, end)`,
    // which the length check at each step keeps.
    unsafe {
        if !header.validate_checksum(addr as *const u8) {
            return;
        }
        if header.length as usize >= mem::size_of::<Dmar>() {
            super::dmar_flags::record(ptr::read_volatile(addr as *const Dmar).flags);
        }
        let mut cursor = addr + mem::size_of::<Dmar>() as u64;
        let end = addr + header.length as u64;

        while cursor + 4 <= end {
            let kind = ptr::read_volatile(cursor as *const u16);
            let length = ptr::read_volatile((cursor + 2) as *const u16);
            if length < 4 || cursor + length as u64 > end {
                // What follows is unread: a unit there would be left off.
                super::dmar_units::left_off();
                break;
            }
            let bytes = core::slice::from_raw_parts(cursor as *const u8, length as usize);
            found_drhd |= entry(kind, cursor, bytes);
            cursor += length as u64;
        }
    }
    if !found_drhd {
        return;
    }
    #[cfg(feature = "nonos-arch-iommu")]
    crate::arch::x86_64::iommu::globals::set_present();
}
