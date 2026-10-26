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

use super::super::core::PagingManager;
use super::teardown::teardown_user_half;
use crate::memory::addr::PhysAddr;
use crate::memory::frame_alloc;
use crate::memory::paging::constants::KERNEL_ASID;
use crate::memory::paging::error::{PagingError, PagingResult};

impl PagingManager {
    pub fn cleanup_address_space(&mut self, asid: u32) -> PagingResult<()> {
        self.cleanup_address_space_keeping(asid, &[])
    }

    /// Frees `asid`'s tables and leaf frames, except the frames in `keep`
    /// (sorted), which another process still maps.
    pub fn cleanup_address_space_keeping(
        &mut self,
        asid: u32,
        keep: &[PhysAddr],
    ) -> PagingResult<()> {
        if let Some(cr3) = self.take_address_space(asid)? {
            free_taken(cr3, keep);
        }
        Ok(())
    }

    /// Take `asid` out of the table: from here no CPU can switch to it or map
    /// into it, so its tables are this caller's alone (`free_taken`).
    pub(in crate::memory::paging::manager) fn take_address_space(
        &mut self,
        asid: u32,
    ) -> PagingResult<Option<PhysAddr>> {
        if asid == KERNEL_ASID {
            return Err(PagingError::KernelSpaceViolation);
        }
        Ok(self.address_spaces.remove(&asid).map(|space| space.cr3_value))
    }
}

/// Zero and free a taken address space's frames and tables, then its root.
/// Called without the manager lock (api/address_space.rs): a gigabyte is
/// 262144 pages zeroed one by one, and under the lock every other CPU's map,
/// fault and switch waited the whole walk, a second and more, long enough for
/// a driver's call to time out.
pub(in crate::memory::paging::manager) fn free_taken(cr3: PhysAddr, keep: &[PhysAddr]) {
    teardown_user_half(cr3, keep);
    // Its TLB tag goes first: freed, the frame can be the next root.
    crate::arch::paging::retire_root(cr3.as_u64());
    let _ = frame_alloc::deallocate_frame(cr3);
}
