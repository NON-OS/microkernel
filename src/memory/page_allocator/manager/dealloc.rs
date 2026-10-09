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

use super::super::error::{PageAllocError, PageAllocResult};
use super::super::types::AllocatedPage;
use super::allocator::PageAllocator;
use crate::memory::addr::VirtAddr;

impl PageAllocator {
    /// Stop tracking the allocation at `va` and hand back its record. The
    /// pages are unmapped by the caller, outside the lock (see `alloc`).
    pub(super) fn take(&mut self, va: VirtAddr) -> PageAllocResult<AllocatedPage> {
        let page_idx = self
            .allocated_pages
            .iter()
            .position(|p| p.virtual_addr == va)
            .ok_or(PageAllocError::PageNotFound)?;
        Ok(self.allocated_pages.remove(page_idx))
    }
}
