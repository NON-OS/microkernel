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

use crate::boot::handoff::BootHandoffV1;

/// The top a 32-bit DMA address can name.
const DMA_CEILING_32BIT: u64 = 0x1_0000_0000;
/// Skip the lowest 16 MiB (legacy/real-mode/SMP-trampoline area) when siting
/// the pool, so it never collides with fixed low-memory uses.
const DMA_POOL_MIN_BASE: u64 = 0x0100_0000;

/// Pick a below-4GB usable region for the DMA pool and return the page-aligned
/// base and page count, or `(0, 0)` when none has room. The size comes from
/// the usable memory below 4 GiB; when no one region holds all of it, the
/// largest one is taken whole rather than leaving 32-bit devices no pool.
pub(super) fn find_low_dma_region(handoff: &BootHandoffV1) -> (u64, usize) {
    let mut low_usable = 0u64;
    let mut best = (0u64, 0usize);
    // SAFETY: the map's pointer and count are the loader's, the same ones
    // setup walked to size the allocator just before; nothing frees them.
    unsafe {
        for (start, end) in handoff.mmap.usable_regions() {
            let (base, pages) = low_window(start, end);
            low_usable += pages as u64 * 0x1000;
            if pages > best.1 {
                best = (base, pages);
            }
        }
    }
    let target = crate::hardware::broker::dma::low32_target_pages(low_usable);
    // SAFETY: as above.
    unsafe {
        for (start, end) in handoff.mmap.usable_regions() {
            let (base, pages) = low_window(start, end);
            if pages >= target {
                return (base, target);
            }
        }
    }
    match crate::hardware::broker::dma::low32_fit(target, best.1) {
        0 => (0, 0),
        pages => (best.0, pages),
    }
}

/// The page-aligned part of `[start, end)` between 16 MiB and 4 GiB.
fn low_window(start: u64, end: u64) -> (u64, usize) {
    let base = (start.max(DMA_POOL_MIN_BASE) + 0xFFF) & !0xFFF;
    let top = end.min(DMA_CEILING_32BIT) & !0xFFF;
    if base >= top {
        return (0, 0);
    }
    (base, ((top - base) / 0x1000) as usize)
}
