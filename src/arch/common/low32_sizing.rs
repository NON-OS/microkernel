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

//! How big the boot makes the low DMA pool, from the memory map rather than a
//! fixed 8 MiB. Only where a boot reserves one (x86_64, aarch64): riscv64 gives
//! no device DMA and has no low pool. Free of kernel state, for the host proofs.
//!
//! One page in 128 of the usable memory below 4 GiB, never under the 2,048
//! pages every machine had before nor over the 8,192 the bitmap tracks. A
//! laptop with 2 GiB usable below 4 GiB gets 16 MiB: room for a Wi-Fi card's
//! rings and buffers and an AHCI HBA without 64-bit addressing at once.

pub(crate) const LOW32_MIN_PAGES: usize = 2048;
/// Smaller than this, a pool would not hold one Wi-Fi card's rings and
/// buffers; the low memory is left to the general allocator instead.
pub(crate) const LOW32_SMALLEST_USEFUL: usize = 256;
const SHARE: u64 = 128;
const PAGE_SIZE: u64 = 4096;

/// The pool wanted for `low_usable` bytes of usable memory below 4 GiB, at
/// most `max`, the pages the pool's bitmap tracks (`LOW32_MAX_PAGES_TRACKED`).
pub(crate) const fn low32_target_pages(low_usable: u64, max: usize) -> usize {
    let pages = (low_usable / SHARE / PAGE_SIZE) as usize;
    if pages < LOW32_MIN_PAGES {
        LOW32_MIN_PAGES
    } else if pages > max {
        max
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
