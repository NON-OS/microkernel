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

//! The arithmetic of where a DMA map may land, kept free of kernel state so the
//! host proofs include it unchanged: whether a run fits below 4 GiB, which a
//! 32-bit device address must, and whether a map may take pages from the
//! reserved low pool.

/// The top a 32-bit device address can name.
pub(crate) const DMA32_CEILING: u64 = 1 << 32;
const PAGE_SIZE: u64 = 4096;

/// Whether `pages` frames starting at `start` end at or below 4 GiB.
pub(crate) const fn fits_below_4g(start: u64, pages: u64) -> bool {
    match pages.checked_mul(PAGE_SIZE) {
        Some(len) => match start.checked_add(len) {
            Some(end) => end <= DMA32_CEILING,
            None => false,
        },
        None => false,
    }
}

/// Whether a map of `pages` may come out of the low pool, which has `free` of
/// its `pool_pages` free. A map for a 32-bit device may take any of it; any
/// other map must leave `reserved` pages behind for those that need them.
pub(crate) const fn low32_may_take(
    free: usize,
    pages: usize,
    pool_pages: usize,
    reserved: usize,
    needs_32bit: bool,
) -> bool {
    if pages == 0 || pages > free {
        return false;
    }
    if needs_32bit {
        return true;
    }
    let floor = if reserved < pool_pages { reserved } else { pool_pages };
    free - pages >= floor
}
