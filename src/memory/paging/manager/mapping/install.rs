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
use super::super::shootdown::flush_tlb_one_smp;
use super::super::tlb_scope::mutation_asid;
use super::tables::{alloc_table, table_at};
use crate::arch::paging::descriptor;
use crate::arch::paging::read_root as read_cr3;
use crate::memory::addr::{PhysAddr, VirtAddr};
use crate::memory::paging::constants::*;
use crate::memory::paging::error::PagingResult;

// CR3 holds the PML4 physical address in bits [51:12]; the low 12 bits are
// flags / PCID. Masking them off yields the active page-table frame.
const CR3_FRAME_MASK: u64 = !0xFFF;

impl PagingManager {
    pub(in crate::memory::paging::manager) fn install_mapping(
        &self,
        va: VirtAddr,
        pa: PhysAddr,
        flags: u64,
    ) -> PagingResult<()> {
        let va_val = va.as_u64();
        let (l4_idx, l3_idx, l2_idx, l1_idx) =
            (pml4_index(va_val), pdpt_index(va_val), pd_index(va_val), pt_index(va_val));
        // The live per-cpu cr3, not the manager's cached field.
        let cr3 = PhysAddr::new(read_cr3() & CR3_FRAME_MASK);
        // SAFETY: eK@nonos.systems - cr3 is the running root, and page
        // tables are reached through the directmap.
        unsafe {
            let l4 = &mut *table_at(cr3);
            if !pte_is_present(l4[l4_idx]) {
                alloc_table(&mut l4[l4_idx])?;
            }
            let l3 = &mut *table_at(PhysAddr::new(pte_address(l4[l4_idx])));
            if !pte_is_present(l3[l3_idx]) {
                alloc_table(&mut l3[l3_idx])?;
            }
            let l2 = &mut *table_at(PhysAddr::new(pte_address(l3[l3_idx])));
            if !pte_is_present(l2[l2_idx]) {
                alloc_table(&mut l2[l2_idx])?;
            }
            let l1 = &mut *table_at(PhysAddr::new(pte_address(l2[l2_idx])));
            l1[l1_idx] = descriptor::leaf(pa.as_u64(), flags);
        }
        let asid = mutation_asid(va, Some(crate::smp::percpu::active_asid()));
        flush_tlb_one_smp(va, asid);
        Ok(())
    }
}
