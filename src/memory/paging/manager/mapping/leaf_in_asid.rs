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

//! The entry that maps one 4 KiB page of an address space that is not
//! running, as the hardware holds it.

use super::tables::table_at;
use crate::memory::addr::{PhysAddr, VirtAddr};
use crate::memory::paging::constants::*;

use super::super::core::PagingManager;

impl PagingManager {
    /// The present 4 KiB leaf entry for `va` in `asid`. None when no entry
    /// maps it, or when a larger block does: a block is not a page a caller
    /// can name a frame of.
    pub(in crate::memory::paging::manager) fn leaf_in_asid(
        &self,
        asid: u32,
        va: VirtAddr,
    ) -> Option<u64> {
        let v = va.as_u64();
        let own = self.address_spaces.get(&asid)?.cr3_value.as_u64();
        let cr3 = PhysAddr::new(crate::arch::paging::root_in(own, v));
        /*
         * SAFETY: eK@nonos.systems - cr3 is one of ours; every table
         * is reached through the directmap and only read here.
         */
        unsafe {
            let e4 = (*table_at(cr3))[pml4_index(v)];
            if !pte_is_present(e4) {
                return None;
            }
            let e3 = (*table_at(PhysAddr::new(pte_address(e4))))[pdpt_index(v)];
            if !pte_is_present(e3) || pte_is_huge(e3) {
                return None;
            }
            let e2 = (*table_at(PhysAddr::new(pte_address(e3))))[pd_index(v)];
            if !pte_is_present(e2) || pte_is_huge(e2) {
                return None;
            }
            let e1 = (*table_at(PhysAddr::new(pte_address(e2))))[pt_index(v)];
            pte_is_present(e1).then_some(e1)
        }
    }
}
