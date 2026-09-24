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

//! Taking a page of the kernel image out of the address space.
//!
//! The manager keeps a record of every page it mapped and `unmap_page`
//! consults that record first. The kernel image is not in it: the bootloader
//! built those mappings before the manager existed. A guard page under a
//! kernel stack is one of them, so unmapping it through the recorded path
//! answers PageNotMapped for a page that is very much mapped, and the guard
//! stays armed on paper only. This path walks the live tables instead.

use super::super::core::PagingManager;
use super::super::pending_flush::PendingFlush;
use crate::memory::addr::{PhysAddr, VirtAddr};
use crate::memory::paging::error::{PagingError, PagingResult};

impl PagingManager {
    /// Unmap a page the bootloader mapped, one the manager has no record of.
    /// The page table entry is cleared in the live root and the TLB flushed,
    /// exactly as for a recorded page; only the bookkeeping is skipped, since
    /// there is none to update.
    pub(in crate::memory::paging::manager) fn unmap_image_page(
        &self,
        virtual_addr: VirtAddr,
    ) -> PagingResult<(PhysAddr, PendingFlush)> {
        if !self.initialized {
            return Err(PagingError::NotInitialized);
        }
        self.remove_mapping(virtual_addr)
    }
}
