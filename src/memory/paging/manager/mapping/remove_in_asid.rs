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

use super::super::core::PagingManager;
use super::super::shootdown::{flush_tlb_one_smp, ASID_KERNEL};
use super::super::tlb_scope::is_kernel_half;
use super::tables::table_at;
use crate::memory::addr::{PhysAddr, VirtAddr};
use crate::memory::paging::constants::*;
use crate::memory::paging::error::{PagingError, PagingResult};

impl PagingManager {
    pub(in crate::memory::paging::manager) fn remove_mapping_in_asid(
        &self,
        asid: u32,
        va: VirtAddr,
    ) -> PagingResult<PhysAddr> {
        let address_space =
            self.address_spaces.get(&asid).ok_or(PagingError::AddressSpaceNotFound)?;
        let cr3 = address_space.cr3_value;
        let va_val = va.as_u64();
        let (l4_idx, l3_idx, l2_idx, l1_idx) =
            (pml4_index(va_val), pdpt_index(va_val), pd_index(va_val), pt_index(va_val));

        // SAFETY: eK@nonos.systems - cr3 is one of ours, and every
        // level is checked present before the next is dereferenced.
        let pa = unsafe {
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
            pa
        };

        /*
         * Unconditional, and skipping it costs most here: the caller frees the
         * frame this returns, so a core still running the guest reads and
         * writes it after the allocator has handed it to somebody else, one
         * process silently editing another's memory with nothing in either of
         * them wrong.
         */
        let scope = if is_kernel_half(va) { ASID_KERNEL } else { asid };
        flush_tlb_one_smp(va, scope);
        Ok(pa)
    }
}
