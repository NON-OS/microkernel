// NØNOS Operating System
// Copyright (C) 2026 NØNOS Contributors
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

// Room for a self-relocating image inside an aligned block (placement.rs).

use uefi::table::boot::{AllocateType, BootServices, MemoryType};

use crate::loader::core::alloc::record_alloc;
use crate::loader::core::constants::{MAX_ALLOCS, PAGE_SIZE};
use crate::loader::errors::{LoaderError, LoaderResult};

/// Any range holding `total` bytes at `virt_min`'s offset inside a whole
/// `align` block, and the base the image goes at.
pub fn allocate_in_block(
    bs: &BootServices,
    virt_min: u64,
    total: usize,
    align: u64,
    allocations: &mut [(u64, usize); MAX_ALLOCS],
    alloc_count: &mut usize,
) -> LoaderResult<u64> {
    let want =
        super::block_layout::span(total as u64, align).ok_or(LoaderError::IntegerOverflow)?;
    let pages = (want as usize).div_ceil(PAGE_SIZE);
    let fail =
        |e: uefi::Error| LoaderError::AllocationFailed { addr: 0, pages, status: e.status() };
    let at =
        bs.allocate_pages(AllocateType::AnyPages, MemoryType::LOADER_DATA, pages).map_err(fail)?;
    record_alloc(allocations, alloc_count, at, pages)?;
    super::block_layout::base(at, virt_min, align).ok_or(LoaderError::IntegerOverflow)
}
