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

//! Finding the entry that governs an address in the running space.

use super::super::super::core::PagingManager;
use crate::memory::addr::VirtAddr;
use crate::memory::layout;
use crate::memory::paging::constants::*;
use crate::memory::paging::error::{PagingError, PagingResult};

/// The leaf entry for an address, and the size bit its level requires.
pub(super) type Leaf = (*mut u64, u64);

fn table(pa: u64) -> *mut [u64; PAGE_TABLE_ENTRIES] {
    (layout::DIRECTMAP_BASE + pa) as *mut [u64; PAGE_TABLE_ENTRIES]
}

impl PagingManager {
    pub(super) fn leaf_for(&self, va: VirtAddr) -> PagingResult<Leaf> {
        let v = va.as_u64();
        let cr3 = self.active_page_table.ok_or(PagingError::NoActivePageTable)?;
        /*
         * SAFETY: eK@nonos.systems - cr3 is the running root, and every
         * level is checked present before the next is dereferenced.
         */
        unsafe {
            let e4 = core::ptr::addr_of_mut!((*table(cr3.as_u64()))[pml4_index(v)]);
            if !pte_is_present(*e4) {
                return Err(PagingError::Pml4NotPresent);
            }
            let e3 = core::ptr::addr_of_mut!((*table(pte_address(*e4)))[pdpt_index(v)]);
            if !pte_is_present(*e3) {
                return Err(PagingError::PdptNotPresent);
            }
            if pte_is_huge(*e3) {
                return Ok((e3, PTE_HUGE_PAGE));
            }
            let e2 = core::ptr::addr_of_mut!((*table(pte_address(*e3)))[pd_index(v)]);
            if !pte_is_present(*e2) {
                return Err(PagingError::PdNotPresent);
            }
            if pte_is_huge(*e2) {
                return Ok((e2, PTE_HUGE_PAGE));
            }
            let e1 = core::ptr::addr_of_mut!((*table(pte_address(*e2)))[pt_index(v)]);
            if !pte_is_present(*e1) {
                return Err(PagingError::PtNotPresent);
            }
            Ok((e1, 0))
        }
    }
}
