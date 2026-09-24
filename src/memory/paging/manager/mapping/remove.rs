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

//! Clearing a leaf in the address space this cpu is running.

use super::super::core::PagingManager;
use super::super::pending_flush::PendingFlush;
use super::super::tlb_scope::mutation_asid;
use super::tables::table_at;
use crate::arch::paging::read_root as read_cr3;
use crate::memory::addr::{PhysAddr, VirtAddr};
use crate::memory::paging::constants::*;
use crate::memory::paging::error::{PagingError, PagingResult};

impl PagingManager {
    pub(in crate::memory::paging::manager) fn remove_mapping(
        &self,
        va: VirtAddr,
    ) -> PagingResult<(PhysAddr, PendingFlush)> {
        let va_val = va.as_u64();
        let (l4_idx, l3_idx, l2_idx, l1_idx) =
            (pml4_index(va_val), pdpt_index(va_val), pd_index(va_val), pt_index(va_val));
        // The live per-cpu root, not the manager's cached one.
        let cr3 = PhysAddr::new(read_cr3() & !0xFFF);
        /*
         * SAFETY: eK@nonos.systems - cr3 is the running root, and every
         * level is checked present before the next is dereferenced.
         */
        unsafe {
            let l4 = &*table_at(cr3);
            if !pte_is_present(l4[l4_idx]) {
                return Err(PagingError::Pml4NotPresent);
            }
            let l3 = &*table_at(PhysAddr::new(pte_address(l4[l4_idx])));
            if !pte_is_present(l3[l3_idx]) {
                return Err(PagingError::PdptNotPresent);
            }
            let l2 = &*table_at(PhysAddr::new(pte_address(l3[l3_idx])));
            if !pte_is_present(l2[l2_idx]) {
                return Err(PagingError::PdNotPresent);
            }
            let l1 = &mut *table_at(PhysAddr::new(pte_address(l2[l2_idx])));
            if !pte_is_present(l1[l1_idx]) {
                return Err(PagingError::PtNotPresent);
            }
            let pa = PhysAddr::new(pte_address(l1[l1_idx]));
            l1[l1_idx] = 0;
            /*
             * Scoped to the asid that owned the mapping, so the ipi
             * reaches the cores running it and no others.
             */
            let asid = mutation_asid(va, Some(crate::smp::percpu::active_asid()));
            Ok((pa, PendingFlush::one(va, asid)))
        }
    }
}
