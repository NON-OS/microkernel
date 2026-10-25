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

//! A holder's MMIO pages pointed at another frame in its own tables, while a
//! thread of it may run on another CPU (L79): the boot framebuffer's pages go
//! to a kernel sink when a GPU driver takes the scanout, in one hold of the
//! paging lock and one invalidation round for the whole run, not one a page.

use super::super::globals::{PAGING_MANAGER, PAGING_STATS};
use crate::arch::run_without_interrupts as without_interrupts;
use crate::memory::addr::{PhysAddr, VirtAddr};
use crate::memory::paging::constants::PAGE_SIZE_4K;
use crate::memory::paging::manager::flush_tlb_range_smp;
use crate::memory::paging::types::{PagePermissions, PageSize};
use crate::smp::lock_responsive;

/// Each of `count` pages from `base` in the tables of `asid` that still shows
/// the frame its grant put there, `from` onward, now shows `to`: user, read
/// and write, never run, cached as the RAM it is. A page the holder unmapped
/// or that maps anything else is left as it is. Returns the pages moved, once
/// every CPU running those tables has dropped what it cached of the run.
pub fn repoint_user_mmio(
    asid: u32,
    base: VirtAddr,
    count: usize,
    from: PhysAddr,
    to: PhysAddr,
) -> usize {
    let perms = PagePermissions::USER | PagePermissions::READ | PagePermissions::WRITE;
    let moved = without_interrupts(|| {
        let mut mgr = lock_responsive(&PAGING_MANAGER);
        let mut moved = 0;
        for i in 0..count {
            let step = (i * PAGE_SIZE_4K) as u64;
            let va = VirtAddr::new(base.as_u64() + step);
            if mgr.translate_in_asid(asid, va) != Some(PhysAddr::new(from.as_u64() + step)) {
                continue;
            }
            match mgr.map_page_in_asid(asid, va, to, perms, PageSize::Size4KiB, &PAGING_STATS) {
                // What each page owes is paid by the one round below.
                Ok(_owed) => moved += 1,
                // Only tables freed meanwhile refuse, and nothing runs on those.
                Err(_) => break,
            }
        }
        moved
    });
    if moved > 0 {
        flush_tlb_range_smp(base, count, asid);
    }
    moved
}
