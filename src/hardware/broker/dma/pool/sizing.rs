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

//! How big the low pool is, from the memory map rather than a fixed 8 MiB,
//! and how full it is. Free of kernel state so the host proofs include it.
//!
//! One page in 128 of the usable memory below 4 GiB, never under the 2,048
//! pages every machine had before nor over the 8,192 the bitmap tracks. A
//! laptop with 2 GiB usable below 4 GiB gets 16 MiB: room for a Wi-Fi card's
//! rings and buffers and an AHCI HBA without 64-bit addressing at once.

pub(crate) const LOW32_MIN_PAGES: usize = 2048;
pub(crate) const LOW32_MAX_PAGES_TRACKED: usize = 8192;
/// Smaller than this, a pool would not hold one Wi-Fi card's rings and
/// buffers; the low memory is left to the general allocator instead.
pub(crate) const LOW32_SMALLEST_USEFUL: usize = 256;
const SHARE: u64 = 128;
const PAGE_SIZE: u64 = 4096;

/// The pool wanted for `low_usable` bytes of usable memory below 4 GiB.
pub(crate) const fn low32_target_pages(low_usable: u64) -> usize {
    let pages = (low_usable / SHARE / PAGE_SIZE) as usize;
    if pages < LOW32_MIN_PAGES {
        LOW32_MIN_PAGES
    } else if pages > LOW32_MAX_PAGES_TRACKED {
        LOW32_MAX_PAGES_TRACKED
    } else {
        pages
    }
}

/// What a region of `region_pages` can hold of a `target`: all of it, as
/// much as fits, or nothing when that is too small to be worth reserving.
pub(crate) const fn low32_fit(target: usize, region_pages: usize) -> usize {
    let pages = if region_pages < target { region_pages } else { target };
    if pages < LOW32_SMALLEST_USEFUL {
        0
    } else {
        pages
    }
}

/// The floor only maps for a 32-bit device (`DMA_MAP_DMA32`) may take: half
/// the pool, as the fixed 1,024 of 2,048 pages was. Other maps stop short of
/// it, so drivers that do not need the pool (xHCI, HDA, NVMe and the like
/// spawn first) cannot leave nothing for one that does.
pub(crate) const fn low32_floor(pool_pages: usize) -> usize {
    pool_pages / 2
}

/// Pressure in quarters: 0 under a quarter used, 4 when full.
pub(crate) const fn pressure_quarter(used: usize, pool_pages: usize) -> usize {
    if pool_pages == 0 {
        return 0;
    }
    let q = used.saturating_mul(4) / pool_pages;
    if q > 4 {
        4
    } else {
        q
    }
}
