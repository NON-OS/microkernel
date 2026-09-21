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

use super::shootdown::flush_tlb_range_smp;
use crate::memory::addr::VirtAddr;

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

    pub(super) fn range(start: VirtAddr, pages: usize, asid: u32) -> Self {
        Self { start, pages, asid }
    }

    pub(super) fn commit(self) {
        if self.pages == 0 {
            return;
        }
        flush_tlb_range_smp(self.start, self.pages, self.asid);
    }
}
