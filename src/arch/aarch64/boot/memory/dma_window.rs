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

use super::dma_place::place;
use super::dma_size::{low_usable, sizes};
use super::regions::bounds;
use super::PAGE_SIZE;
use crate::arch::aarch64::boot::info::{BootInfo, MemoryType};

/// Off the first 16 MiB, where a board whose RAM starts at zero keeps
/// firmware and the secondary-CPU trampoline.
const FLOOR: u64 = 0x0100_0000;
/// The allocator's span, the image, and room for every reserved range.
const MAX_AVOID: usize = 32;

/// Where the below-4 GiB DMA pool goes. `carved` means it lies inside the
/// allocator's span, which the caller must then reserve.
pub(super) struct LowPool {
    pub(super) base: u64,
    pub(super) pages: usize,
    pub(super) carved: bool,
}

/// A pool clear of the allocator's span `main`, the kernel `image` and every
/// range the tree reserved, or, failing that, one carved from inside `main`
/// clear of the image and the reservations, at the largest size `dma_size`
/// offers that fits. `None` when RAM below 4 GiB has no room either way.
pub(super) fn find_low_dma_region(
    info: &BootInfo,
    main: (u64, u64),
    image: (u64, u64),
) -> Option<LowPool> {
    let mut available = [(0u64, 0u64); 8];
    let mut n = 0;
    let usable = info.memory_map().iter().filter(|r| r.region_type == MemoryType::Available);
    for (slot, range) in available.iter_mut().zip(usable.filter_map(bounds)) {
        *slot = range;
        n += 1;
    }
    let reserved = info.reserved.ranges();
    let mut avoid = [(0u64, 0u64); MAX_AVOID];
    if reserved.len() + 2 > MAX_AVOID {
        return None;
    }
    avoid[0] = main;
    avoid[1] = image;
    avoid[2..2 + reserved.len()].copy_from_slice(reserved);
    let avoid = &avoid[..2 + reserved.len()];
    for want in sizes(low_usable(&available[..n], FLOOR)) {
        let pages = (want / PAGE_SIZE) as usize;
        if let Some(base) = place(&available[..n], avoid, FLOOR, want) {
            return Some(LowPool { base, pages, carved: false });
        }
        if let Some(base) = place(&[main], &avoid[1..], FLOOR, want) {
            return Some(LowPool { base, pages, carved: true });
        }
    }
    None
}
