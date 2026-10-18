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

//! Where a grant's frames come from.

use super::super::pool;
use crate::hardware::broker::dma::flags::{DMA_MAP_DMA32, DMA_MAP_HIGH};
use crate::hardware::broker::dma::sources::{sources, Source};
use crate::memory::phys::{alloc_contiguous, AllocFlags};

// Where the frames come from, by request flag:
//   HIGH   the display pool, else anywhere, highest first
//   DMA32  the low pool, else below 4 GiB, else no memory: the device
//          names buffers with 32-bit addresses (an AHCI HBA without
//          CAP.S64A) and a frame above 4 GiB would be truncated
//   none   the low pool, else below 4 GiB, else anywhere, so a device
//          with no IOMMU in front of it still gets low memory when
//          there is some
// An uncached grant for a device that does not snoop never comes here:
// alloc.rs takes it from the uncached pool or refuses it (uncached.rs).
pub(super) fn take(pages: usize, dma_flags: u32) -> Option<u64> {
    let base = AllocFlags::DMA | AllocFlags::ZERO;
    let dma32 = dma_flags & DMA_MAP_DMA32 != 0;
    // The order is sources.rs's, pure (D6b). A map that is not DMA32 stops
    // short of the low pool's floor, kept for those that are.
    sources(dma_flags & DMA_MAP_HIGH != 0, dma32).iter().find_map(|source| match source {
        Source::DisplayPool => pool::alloc(pages),
        Source::HighFirst => alloc_contiguous(pages, base | AllocFlags::HIGH),
        Source::LowPool => pool::low32_alloc(pages, dma32),
        Source::Below4G => alloc_contiguous(pages, base | AllocFlags::DMA32),
        Source::Anywhere => alloc_contiguous(pages, base),
    })
}
