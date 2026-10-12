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

//! RAM identity mapped in 2 MiB blocks, the kernel image's blocks in pages.

use super::super::boot::info::MemoryType as BootMemoryType;
use super::geometry::{BLOCK_2M, ENTRIES, GIB};
use super::l2_pool::l2_under;
use super::{image_map, state, PageAttributes};

/// Identity map `[base, base + size)`. Each block goes in the level 2 table
/// of the gigabyte it is in, so a region may span any number of them.
/// `Err` carries the first address left unmapped when the pool ran out.
///
/// # Safety
///
/// Boot CPU, before these tables are live.
pub(super) unsafe fn map_range(base: u64, size: u64, kind: BootMemoryType) -> Result<(), u64> {
    let data_attrs = region_attrs(kind);
    let code_attrs = PageAttributes::kernel_code();
    let mut phys = base & !(BLOCK_2M - 1);
    let end = base.saturating_add(size);
    while phys < end {
        let l1_idx = (phys / GIB) as usize;
        let l2 = match l1_idx < ENTRIES {
            true => l2_under(state::l1(), l1_idx).ok_or(phys)?,
            false => return Err(phys),
        };
        let l2_idx = ((phys / BLOCK_2M) % ENTRIES as u64) as usize;
        /*
         * The image sits inside a region the firmware calls Available, and its
         * text and read-only data share a block, so a block that touches it is
         * described page by page. Only when no level 3 table covers the block is
         * it mapped whole and executable, which the kernel section check then
         * reports as W^X not held.
         */
        let paged = image_map::overlaps_image(phys)
            && image_map::map_image_block(l2, l1_idx, l2_idx, phys, &data_attrs);
        if !paged {
            let attrs = if image_map::overlaps_image(phys) { &code_attrs } else { &data_attrs };
            l2.set_block(l2_idx, phys, attrs);
        }
        phys = phys.saturating_add(BLOCK_2M);
    }
    Ok(())
}

fn region_attrs(kind: BootMemoryType) -> PageAttributes {
    match kind {
        BootMemoryType::Kernel => PageAttributes::kernel_code(),
        BootMemoryType::DeviceMemory => PageAttributes::device(),
        BootMemoryType::Available => PageAttributes::kernel_data(),
        _ => PageAttributes::kernel_rodata(),
    }
}
