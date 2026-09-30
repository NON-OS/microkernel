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
/// base and page count, or `(0, 0)` when none has room.
pub(super) fn find_low_dma_region(handoff: &BootHandoffV1) -> (u64, usize) {
    let want_pages = crate::hardware::broker::dma::low32_capacity_pages();
    let want_bytes = (want_pages as u64) * 0x1000;
    unsafe {
        for (start, end) in handoff.mmap.usable_regions() {
            let base = ((start.max(DMA_POOL_MIN_BASE)) + 0xFFF) & !0xFFF;
            let top = end.min(DMA_CEILING_32BIT);
            if base < top && top - base >= want_bytes {
                return (base, want_pages);
            }
        }
    }
    (0, 0)
}
