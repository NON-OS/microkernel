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

use super::regions::bounds;
use super::PAGE_SIZE;
use crate::arch::aarch64::boot::info::{BootInfo, MemoryType};
use crate::hardware::broker::dma;

/// The top a 32-bit DMA address can name.
const DMA_CEILING_32BIT: u64 = 0x1_0000_0000;
/// Keep the pool clear of the bottom of RAM, where the kernel image and the
/// secondary-CPU trampoline live.
const DMA_POOL_MIN_BASE: u64 = 0x0100_0000;

/// A below-4 GiB window that overlaps neither the general allocator's range
/// nor anything the device tree reserved, or `(0, 0)` when there is none.
/// Overlapping the allocator would let both hand out the same frame.
pub(super) fn find_low_dma_region(info: &BootInfo, main_start: u64, main_end: u64) -> (u64, usize) {
    let want_pages = dma::low32_default_pages();
    let want_bytes = (want_pages as u64) * PAGE_SIZE;

    for region in info.memory_map().iter() {
        if region.region_type != MemoryType::Available {
            continue;
        }
        let Some((start, end)) = bounds(region) else {
            continue;
        };
        let base = start.max(DMA_POOL_MIN_BASE);
        let top = end.min(DMA_CEILING_32BIT);
        if base >= top || top - base < want_bytes {
            continue;
        }
        let clear = base + want_bytes <= main_start || base >= main_end;
        if clear && !info.reserved.overlaps(base, base + want_bytes) {
            return (base, want_pages);
        }
    }
    (0, 0)
}
