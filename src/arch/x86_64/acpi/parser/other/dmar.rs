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
use super::dmar_namespace_store::{record_andd, ANDD_TYPE};
use super::dmar_rmrr::RMRR_TYPE;
use super::dmar_rmrr_store::{record_rmrr, rmrr_cut_short};
use super::dmar_units::record_drhd;
use super::dmar_walk::walk_structures;
use crate::arch::x86_64::acpi::tables::{Drhd, SdtHeader, SIG_DMAR};

const DRHD_TYPE: u16 = 0;

pub fn parse_dmar(registry: &mut TableRegistry) {
    let Some(&addr) = registry.tables.get(&SIG_DMAR) else { return };
    let Some(addr) = super::super::phys::directmap(addr) else { return };
    // SAFETY: eK@nonos.systems - the registry holds tables firmware published
    // and the directmap covers them; the header is read before the length it
    // names is trusted, and the checksum refuses one past MAX_TABLE_BYTES.
    let table = unsafe {
        let header = ptr::read_volatile(addr as *const SdtHeader);
        if !header.validate_checksum(addr as *const u8) {
            return;
        }
        core::slice::from_raw_parts(addr as *const u8, header.length as usize)
    };
    let mut found_drhd = false;
    let walked = walk_structures(table, |kind, bytes| match kind {
        DRHD_TYPE if bytes.len() >= mem::size_of::<Drhd>() => {
            // SAFETY: eK@nonos.systems - `bytes` holds a whole Drhd, packed.
            let drhd = unsafe { ptr::read_unaligned(bytes.as_ptr() as *const Drhd) };
            found_drhd |= record_drhd(drhd, bytes);
        }
        RMRR_TYPE => record_rmrr(bytes),
        ANDD_TYPE => record_andd(bytes),
        _ => {}
    });
    // A cut after the RMRRs hid none of them; before, any could be missing (L67).
    if walked.cut_short {
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
