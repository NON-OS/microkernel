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

use crate::memory::addr::VirtAddr;
use crate::memory::paging::manager::api::translate_in_asid;

use super::pcb::ProcessControlBlock;

const PAGE: u64 = 4096;
/* Bound on occupied ranges skipped before the reservation gives up. */
const MAX_ATTEMPTS: usize = 64;

impl ProcessControlBlock {
    /*
     * Take `pages` pages of VA from this process's mmap allocator and make
     * sure none of them is already present in `asid`. The allocator only
     * knows the ranges it handed out itself: the ELF image, the stack and
     * fixed-address mappings are invisible to it. A range found occupied
     * stays reserved, so the allocator never offers it again, and the next
     * range is tried. Callers therefore never map over a live PTE.
     */
    pub fn reserve_unmapped(&self, pages: u64, asid: u32) -> Option<u64> {
        for _ in 0..MAX_ATTEMPTS {
            let base = self.mmap_va.lock().reserve(pages)?;
            if range_unmapped(asid, base, pages) {
                return Some(base);
            }
        }
        None
    }
}

/* True when no page of [base, base + pages * 4 KiB) is present in `asid`. */
fn range_unmapped(asid: u32, base: u64, pages: u64) -> bool {
    (0..pages).all(|i| translate_in_asid(asid, VirtAddr::new(base + i * PAGE)).is_none())
}
