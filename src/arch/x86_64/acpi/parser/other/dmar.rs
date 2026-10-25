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

use super::dmar_entry::entry;
use super::dmar_rmrr_store::rmrr_cut_short;
use super::dmar_walk::walk_structures;
use crate::arch::x86_64::acpi::parser::phys::mapped_table;
use crate::arch::x86_64::acpi::parser::state::TableRegistry;
use crate::arch::x86_64::acpi::tables::{Dmar, SIG_DMAR};

pub fn parse_dmar(registry: &mut TableRegistry) {
    let Some(&addr) = registry.tables.get(&SIG_DMAR) else { return };
    let Some((addr, header)) = mapped_table(addr) else { return };
    // SAFETY: eK@nonos.systems - `addr` is the directmapped table ACPI
    // published, mapped for its whole length (mapped_table); the checksum
    // refuses a length under the header or past MAX_TABLE_BYTES before the
    // slice is made over it.
    let table = unsafe {
        if !header.validate_checksum(addr as *const u8) {
            return;
        }
        core::slice::from_raw_parts(addr as *const u8, header.length as usize)
    };
    if table.len() >= mem::size_of::<Dmar>() {
        // SAFETY: eK@nonos.systems - `table` holds a whole Dmar, packed.
        let dmar = unsafe { ptr::read_unaligned(table.as_ptr() as *const Dmar) };
        super::dmar_flags::record(dmar.flags);
        super::dmar_flags::record_width(dmar.host_address_width);
    }
    let mut found_drhd = false;
    let walked = walk_structures(table, |kind, bytes| found_drhd |= entry(kind, bytes));
    if walked.cut_short {
        // What follows is unread: a unit there would be left off (K11). A cut
        // after the RMRRs hid none of them; before, any could be missing (L67).
        super::dmar_units::left_off();
        let rmrrs = if walked.past_rmrr { "kept" } else { "withheld" };
        crate::log::info!("[VT-D] DMAR cut short at a bad structure; RMRR mappings {}", rmrrs);
        if !walked.past_rmrr {
            rmrr_cut_short();
        }
    }
    if !found_drhd {
        return;
    }
    #[cfg(feature = "nonos-arch-iommu")]
    crate::arch::x86_64::iommu::globals::set_present();
}
