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

// Where the kernel image goes and where it is entered. Three modes:
//
//   1. Upper-half: the ELF declares an upper-half window; any physical
//      range will do, and the entry stays the virtual address it names.
//   2. Low-half fixed: phys == virt, so UEFI is asked for the link address.
//   3. Self-relocating (an architecture that sets arch::cpu::IMAGE_ALIGN,
//      aarch64): the image runs wherever it is put, as long as it keeps its
//      offset inside an `align` block (the arm64 Image's text offset). Any
//      physical range is taken, the image laid at that offset in the first
//      whole block, and the entry moved with it. The link address is never
//      asked for: firmware may already hold it.

use uefi::table::boot::BootServices;

use crate::loader::core::constants::{MAX_ALLOCS, PAGE_SIZE};
use crate::loader::errors::{LoaderError, LoaderResult};

use super::allocate_block::allocate_in_block;
use super::allocate_image::allocate_image;
use super::entry_in_range::entry_in_range;

/// Where the image lies and where it is entered.
pub struct Placed {
    pub phys_base: u64,
    pub entry: usize,
    pub virt_base: u64,
}

#[allow(clippy::too_many_arguments)]
pub fn place_image(
    bs: &BootServices,
    virt_min: u64,
    e_entry: usize,
    total_bytes: usize,
    upper_half: bool,
    allocations: &mut [(u64, usize); MAX_ALLOCS],
    alloc_count: &mut usize,
) -> LoaderResult<Placed> {
    let pages = total_bytes.div_ceil(PAGE_SIZE);
    let (phys_base, entry) = match crate::arch::cpu::IMAGE_ALIGN {
        Some(align) if !upper_half => {
            let base =
                allocate_in_block(bs, virt_min, total_bytes, align, allocations, alloc_count)?;
            let offset =
                e_entry.checked_sub(virt_min as usize).ok_or(LoaderError::EntryNotInRange)?;
            (base, base as usize + offset)
        }
        _ => (allocate_image(bs, virt_min, pages, upper_half, allocations, alloc_count)?, e_entry),
    };
    if !entry_in_range(entry, upper_half, virt_min, phys_base, total_bytes) {
        return Err(LoaderError::EntryNotInRange);
    }
    Ok(Placed { phys_base, entry, virt_base: if upper_half { virt_min } else { 0 } })
}
