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

//! A TLB invalidation owed by a page-table change, carried out of the
//! paging lock and paid once the lock is released.
//!
//! Every mutation site writes its entries under `PAGING_MANAGER` and hands
//! back one of these instead of flushing in place. A remote flush waits for
//! every target to acknowledge; waiting with the manager lock held made every
//! other cpu that wanted the lock (a context switch, a fault, a mapping) spin
//! for as long as the slowest target took, and any of them spinning on a plain
//! lock with interrupts masked could not answer, which turned a slow round
//! into a timeout. The caller still commits before it returns, so nothing that
//! depends on the invalidation (freeing the old frame, handing the address
//! range to someone else) can run before the round completes.

use super::shootdown::flush_tlb_range_smp;
use crate::memory::addr::VirtAddr;
use crate::memory::paging::tlb;

#[must_use = "a pending TLB invalidation must be committed after the paging lock is released"]
pub(super) struct PendingFlush {
    start: VirtAddr,
    pages: usize,
    asid: u32,
}

impl PendingFlush {
    pub(super) fn one(va: VirtAddr, asid: u32) -> Self {
        Self { start: va, pages: 1, asid }
    }
    /// Nothing owed.
    pub(super) fn none() -> Self {
        Self { start: VirtAddr::new(0), pages: 0, asid: 0 }
    }

    /// Drop `va` on this cpu now, under the lock and with interrupts masked
    /// as before, and owe only the other cpus. Between the unlock and the
    /// commit this cpu may take an interrupt and run another thread of the
    /// same address space, which must not see the old entry; peers could
    /// already see it until they acknowledge, lock or no lock.
    pub(super) fn one_local_now(va: VirtAddr, asid: u32) -> Self {
        tlb::invalidate_page(va);
        Self::one(va, asid)
    }

    pub(super) fn commit(self) {
        if self.pages == 0 {
            return;
        }
        flush_tlb_range_smp(self.start, self.pages, self.asid);
    }

    /// Folds another invalidation into this one, widening the page count.
    /// The caller must only absorb tokens for pages contiguous with this
    /// token's range and in the same address space; `unmap_range` walks a
    /// single contiguous mapping, so both hold there.
    pub(super) fn absorb(&mut self, other: PendingFlush) {
        debug_assert_eq!(self.asid, other.asid);
        self.pages += other.pages;
    }
}
