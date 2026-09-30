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

use core::sync::atomic::{AtomicU64, Ordering};

use crate::memory::addr::VirtAddr;

use super::super::core::PagingManager;
use super::super::pending_flush::PendingFlush;
use crate::memory::paging::constants::{
    page_align_down, pte_is_user, pte_is_writable, PAGE_SIZE_4K,
};
use crate::memory::paging::error::{PagingError, PagingResult};
use crate::memory::paging::stats::PagingStatistics;
use crate::memory::paging::types::{PagePermissions, PageSize};
use crate::memory::{frame_alloc, layout};
use crate::smp::MAX_CPUS;

impl PagingManager {
    pub(super) fn handle_cow_fault(
        &mut self,
        virtual_addr: VirtAddr,
        stats: &PagingStatistics,
    ) -> PagingResult<PendingFlush> {
        // A write fault on a *present* page is legitimate only when that page
        // was mapped copy-on-write. Any other present+write fault is a
        // protection violation: a write to a read-only page (a RELRO'd GOT,
        // .rodata, or code) or a supervisor write to a kernel page. Neither may
        // be silently promoted to writable, so fail closed here and let the
        // fault handler kill the offending capsule (or halt on a kernel fault).
        if !layout::in_user_space(virtual_addr.as_u64()) {
            return Err(PagingError::UnhandledPageFault);
        }
        /*
         * The fault reported a read-only entry when it was taken, not now.
         * Two threads of one process on two CPUs can write the same
         * copy-on-write page at once; the second waits on the manager lock
         * while the first copies it. The record below then no longer says
         * copy-on-write, and the second thread was killed for a protection
         * fault on a page it may write. The caller holds the lock, so the
         * entry cannot change before the copy below; writable means the
         * faulting access is retried, once (see `retry_once`).
         */
        let page_addr = page_align_down(virtual_addr.as_u64());
        let writable =
            self.leaf_entry(virtual_addr).is_ok_and(|e| pte_is_writable(e) && pte_is_user(e));
        if retry_once(page_addr, writable) {
            return Ok(PendingFlush::none());
        }
        let original =
            self.mappings.get(&page_addr).ok_or(PagingError::UnhandledPageFault)?.permissions;
        if !original.contains(PagePermissions::COW) {
            return Err(PagingError::UnhandledPageFault);
        }

        let new_frame = frame_alloc::allocate_frame().ok_or(PagingError::FrameAllocationFailed)?;

        if let Ok(original_pa) = self.translate_address(virtual_addr) {
            unsafe {
                let src_va = layout::DIRECTMAP_BASE + original_pa.as_u64();
                let dst_va = layout::DIRECTMAP_BASE + new_frame.as_u64();
                core::ptr::copy_nonoverlapping(
                    src_va as *const u8,
                    dst_va as *mut u8,
                    PAGE_SIZE_4K,
                );
            }
        }

        // Resolve the copy-on-write: drop the COW marker and grant the deferred
        // write, preserving the original permissions (never fabricating USER).
        let permissions = original.remove(PagePermissions::COW).insert(PagePermissions::WRITE);
        /*
         * A frame change on a present entry: another thread of the process on
         * another cpu may cache the read-only entry for the shared frame and
         * would go on reading it, so the flush this returns is a real one.
         */
        self.map_page(virtual_addr, new_frame, permissions, PageSize::Size4KiB, stats)
    }
}

static RETRIED: [AtomicU64; MAX_CPUS] = [const { AtomicU64::new(0) }; MAX_CPUS];

/*
 * Whether to send the access back to run again. A write that faults on an
 * entry already writable was racing another CPU's copy and succeeds when run
 * again. One that faults a second time on the same page is not such a race
 * (a supervisor write the access-prevention bit refused, say) and is failed
 * as before rather than retried for ever. A lone CPU cannot race itself, so
 * the single-CPU image fails every such fault as it always did.
 */
fn retry_once(page_addr: u64, writable: bool) -> bool {
    if !cfg!(feature = "nonos-smp") {
        return false;
    }
    let slot = &RETRIED[crate::smp::cpu_id() % MAX_CPUS];
    if writable && slot.swap(page_addr, Ordering::Relaxed) != page_addr {
        return true;
    }
    slot.store(0, Ordering::Relaxed);
    false
}
