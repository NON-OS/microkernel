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

use super::super::core::PagingManager;
use super::super::pending_flush::PendingFlush;
use crate::memory::paging::constants::PAGE_SIZE_4K;
use crate::memory::paging::error::{PagingError, PagingResult};
use crate::memory::paging::stats::PagingStatistics;
use crate::memory::paging::types::{PagePermissions, PageSize};
use crate::memory::{frame_alloc, layout};

impl PagingManager {
    pub(super) fn handle_demand_fault(
        &mut self,
        virtual_addr: VirtAddr,
        stats: &PagingStatistics,
    ) -> PagingResult<PendingFlush> {
        let pid = crate::process::current_pid().unwrap_or(0);
        if super::demand_refuse::refused(virtual_addr.as_u64(), pid) {
            return Err(PagingError::UnhandledPageFault);
        }

        /*
         * The fault reported the page absent when it was taken, not now. Two
         * threads of one process on two CPUs can fault on the same page at
         * once; the second waits on the manager lock while the first fills
         * it. Filling again would put a fresh zero page over the one the
         * first thread may already have written, and leak it. The caller
         * holds the lock, so nothing can change the entry between this check
         * and the fill below; present means the faulting access is retried.
         */
        if self.leaf_entry(virtual_addr).is_ok() {
            return Ok(PendingFlush::none());
        }

        /*
         * Charge the page against the faulting process's demand budget. A
         * runaway capsule is refused here and killed by the fault path instead
         * of exhausting physical memory.
         */
        if !super::demand_cap::charge(pid) {
            return Err(PagingError::UnhandledPageFault);
        }

        let new_frame = frame_alloc::allocate_frame().ok_or(PagingError::FrameAllocationFailed)?;

        unsafe {
            let va = layout::DIRECTMAP_BASE + new_frame.as_u64();
            core::ptr::write_bytes(va as *mut u8, 0, PAGE_SIZE_4K);
        }

        let permissions = PagePermissions::READ | PagePermissions::WRITE | PagePermissions::USER;
        /*
         * The entry is absent (checked above, under the lock), so the install
         * owes no remote flush.
         */
        self.map_page(virtual_addr, new_frame, permissions, PageSize::Size4KiB, stats)
    }
}
