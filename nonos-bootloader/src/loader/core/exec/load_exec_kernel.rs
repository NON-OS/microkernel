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

// Loading an ET_EXEC kernel, placed one of three ways:
//   1. low half (legacy): at the fixed address the ELF declares, phys == virt;
//   2. upper half (x86_64): any contiguous phys range, each PT_LOAD at
//      phys + (p_vaddr - min_vaddr), its (phys, virt, size, flags) kept for
//      the paging stage's mappings before the CR3 swap;
//   3. a self-relocating image (aarch64): an aligned block (placement.rs),
//      across the footprint its header claims (footprint.rs).

use crate::loader::errors::LoaderResult;
use crate::loader::image::KernelImage;
use crate::loader::types::memory;
use crate::log::logger::log_error;

use crate::loader::core::alloc::free_all;
use crate::loader::core::constants::MAX_ALLOCS;
use crate::loader::core::types::ValidationResult;

use super::footprint::footprint;
use super::image_record::image_record;
use super::load_segments::load_segments;
use super::placement::place_image;

pub fn load_exec_kernel(
    bs: &uefi::table::boot::BootServices,
    payload: &[u8],
    v: &ValidationResult,
) -> LoaderResult<KernelImage> {
    let virt_min = v.min_addr;
    let total_bytes = footprint(payload, v, (v.max_addr - v.min_addr) as usize)?;
    let upper_half = memory::is_upper_half(virt_min);

    let mut allocations: [(u64, usize); MAX_ALLOCS] = [(0, 0); MAX_ALLOCS];
    let mut alloc_count: usize = 0;

    let placed = match place_image(
        bs,
        virt_min,
        v.elf.header.e_entry as usize,
        total_bytes,
        upper_half,
        &mut allocations,
        &mut alloc_count,
    ) {
        Ok(p) => p,
        Err(e) => {
            free_all(bs, &allocations, alloc_count);
            log_error("loader", "ELF entry not in loaded segments, or no room for the image");
            return Err(e);
        }
    };
    let segments = load_segments(payload, v, placed.phys_base, virt_min);
    Ok(image_record(&placed, total_bytes, segments, payload.len()))
}
