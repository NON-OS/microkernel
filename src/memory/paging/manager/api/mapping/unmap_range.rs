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

use super::super::super::pending_flush::PendingFlush;
use super::super::globals::{PAGING_MANAGER, PAGING_STATS};
use crate::arch::run_without_interrupts as without_interrupts;
use crate::memory::addr::VirtAddr;
use crate::memory::paging::constants::{pages_needed, PAGE_SIZE_4K};
use crate::memory::paging::error::PagingResult;
use crate::smp::lock_responsive;

// Multi-page unmap. Walks 4 KiB pages from `virtual_addr` for `size`
// bytes (rounded up), taking the paging lock once and folding every
// page's invalidation into a single cross-CPU rendezvous instead of
// one per page. Stops at the first failure; the pages already
// unmapped by then are still invalidated before the error returns.
pub fn unmap_range(virtual_addr: VirtAddr, size: usize) -> PagingResult<()> {
    let count = pages_needed(size);
    if count == 0 {
        return Ok(());
    }

    let (flush, outcome) = without_interrupts(|| {
        let mut mgr = lock_responsive(&PAGING_MANAGER);
        let mut flush: Option<PendingFlush> = None;
        let mut outcome = Ok(());
        for i in 0..count {
            let va = VirtAddr::new(virtual_addr.as_u64() + (i * PAGE_SIZE_4K) as u64);
            match mgr.unmap_page(va) {
                Ok((_, perms, page_size, page_flush)) => {
                    PAGING_STATS.record_unmapping(perms, page_size);
                    match flush.as_mut() {
                        Some(f) => f.absorb(page_flush),
                        None => flush = Some(page_flush),
                    }
                }
                Err(e) => {
                    outcome = Err(e);
                    break;
                }
            }
        }
        (flush, outcome)
    });

    if let Some(f) = flush {
        f.commit();
    }
    outcome
}
