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
        keep: &[crate::memory::addr::PhysAddr],
    ) -> PagingResult<()> {
        if asid == KERNEL_ASID {
            return Err(PagingError::KernelSpaceViolation);
        }
        if let Some(address_space) = self.address_spaces.remove(&asid) {
            teardown_user_half(address_space.cr3_value, keep);
            let _ = frame_alloc::deallocate_frame(address_space.cr3_value);
        }
        Ok(())
    }
}
