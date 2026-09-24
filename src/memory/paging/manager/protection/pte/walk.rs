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

//! Rewriting the flags of a mapping in the running address space.

use super::super::super::core::PagingManager;
use super::super::super::shootdown::flush_tlb_one_smp;
use super::super::super::tlb_scope::mutation_asid;
use crate::arch::paging::descriptor;
use crate::memory::addr::VirtAddr;
use crate::memory::paging::constants::pte_address;
use crate::memory::paging::error::PagingResult;

impl PagingManager {
    pub(in super::super) fn update_pte(&self, va: VirtAddr, new_flags: u64) -> PagingResult<()> {
        let (entry, size_bit) = self.leaf_for(va)?;
        // SAFETY: eK@nonos.systems - `leaf_for` hands back a pointer to
        // a present entry in a live table, reached through the directmap.
        unsafe {
            *entry = descriptor::leaf(pte_address(*entry), new_flags | size_bit);
        }
        // One invalidation for one write.
        flush_tlb_one_smp(va, mutation_asid(va, Some(crate::smp::percpu::active_asid())));
        Ok(())
    }
}
