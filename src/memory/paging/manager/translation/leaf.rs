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

//! The entry the hardware would use for an address, read from the live tables.
//!
//! The manager keeps a record of every mapping it installed and answers
//! permission queries from it. That record cannot answer for the kernel
//! image, the boot stacks, or anything else the bootloader mapped before the
//! manager existed, and it does not say so: it returns nothing, which reads
//! the same as a page that is not mapped at all. The W^X check over the
//! kernel sections came back 0 of 4 for exactly that reason, on a kernel
//! whose bootloader maps every segment with the correct permissions.
//!
//! So this walks the tables instead, from the live root, and hands back the
//! leaf. Blocks at level 3 and level 2 are leaves too, and a kernel mapped
//! with 2 MiB pages would otherwise read as unmapped.

use super::super::core::PagingManager;
use crate::arch::paging::read_root;
use crate::memory::addr::{PhysAddr, VirtAddr};
use crate::memory::layout;
use crate::memory::paging::constants::*;
use crate::memory::paging::error::{PagingError, PagingResult};

fn table_at(pa: u64) -> *const [u64; PAGE_TABLE_ENTRIES] {
    (layout::DIRECTMAP_BASE + pa) as *const [u64; PAGE_TABLE_ENTRIES]
}

impl PagingManager {
    /// The page-table entry that maps `virtual_addr`, whatever level it lives
    /// at. The root comes from the register rather than the cached field, so
    /// the answer is about the address space this CPU is running in.
    pub fn leaf_entry(&self, virtual_addr: VirtAddr) -> PagingResult<u64> {
        let va = virtual_addr.as_u64();
        let root = PhysAddr::new(read_root() & !0xFFF);
        let descend = [
            (pml4_index(va), PagingError::Pml4NotPresent),
            (pdpt_index(va), PagingError::PdptNotPresent),
            (pd_index(va), PagingError::PdNotPresent),
            (pt_index(va), PagingError::PtNotPresent),
        ];
        let mut table = root.as_u64();
        for (level, &(index, absent)) in descend.iter().enumerate() {
            /*
             * SAFETY: every table address reached here came from a present
             * entry one level up, or from the root register, and the
             * directmap covers all of physical memory for reads.
             */
            let entry = unsafe { (*table_at(table))[index] };
            if !pte_is_present(entry) {
                return Err(absent);
            }
            // A block at level 1 or 2 is the leaf, as is any entry at level 3.
            if level == descend.len() - 1 || pte_is_huge(entry) {
                return Ok(entry);
            }
            table = pte_address(entry);
        }
        Err(PagingError::PtNotPresent)
    }
}
