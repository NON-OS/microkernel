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

//! Dropping a mapping from the running address space and its record.

use super::super::core::PagingManager;
use crate::memory::addr::{PhysAddr, VirtAddr};
use crate::memory::paging::constants::page_align_down;
use crate::memory::paging::error::{PagingError, PagingResult};
use crate::memory::paging::types::{PagePermissions, PageSize};

impl PagingManager {
    pub fn unmap_page(
        &mut self,
        virtual_addr: VirtAddr,
    ) -> PagingResult<(PhysAddr, PagePermissions, PageSize)> {
        if !self.initialized {
            return Err(PagingError::NotInitialized);
        }
        let page_addr = page_align_down(virtual_addr.as_u64());
        let mapping = self.mappings.remove(&page_addr).ok_or(PagingError::PageNotMapped)?;
        let physical_addr = self.remove_mapping(virtual_addr)?;
        Ok((physical_addr, mapping.permissions, mapping.size))
    }
}
