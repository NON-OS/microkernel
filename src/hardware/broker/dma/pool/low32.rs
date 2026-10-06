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

//! A reserved pool of below-4GB memory for 32-bit DMA devices. Regular
//! allocations consume low memory bottom-up, so by the time a wifi driver
//! initialises there is often nothing left below 4GB; reserving this pool
//! early guarantees a home for buffers named by a 32-bit descriptor. Sized
//! for device rings and small staging buffers, not bulk data (storage is
//! 64-bit capable and never comes here).

use super::bitmap::WORD_BITS;
use super::sizing::{low32_floor, LOW32_MAX_PAGES_TRACKED};
use spin::Mutex;

const LOW32_WORDS: usize = LOW32_MAX_PAGES_TRACKED / WORD_BITS;
/// The largest single request served from the low pool; larger DMA is 64-bit
/// capable and goes to the general allocator instead of draining the reserve.
pub(super) const LOW32_MAX_PAGES: usize = 256; // 1MB

pub(super) struct Low32Pool {
    pub(super) base: u64,
    pub(super) pages: usize,
    pub(super) used: [u64; LOW32_WORDS],
}

pub(super) static LOW32_POOL: Mutex<Low32Pool> =
    Mutex::new(Low32Pool { base: 0, pages: 0, used: [0; LOW32_WORDS] });

/// The pool asked for by a boot path that does not size it from the memory
/// map (the device tree path on aarch64): the 8 MiB every machine had before.
#[cfg(not(target_arch = "x86_64"))]
pub(crate) const fn low32_default_pages() -> usize {
    super::sizing::LOW32_MIN_PAGES
}

/// Record the below-4GB physical range `[base, base + pages*PAGE)` as the DMA
/// pool. The caller carves this from a low usable memory region the main
/// allocator does not manage, so the two never hand out the same frame. `base`
/// zero (no low region found) leaves the pool unavailable.
pub(crate) fn init_low32_pool(base: u64, pages: usize) {
    let mut pool = LOW32_POOL.lock();
    if pool.pages != 0 {
        return;
    }
    let pages = pages.min(LOW32_MAX_PAGES_TRACKED);
    if base == 0 || pages == 0 {
        crate::sys::serial::println(b"[DMA] low32 pool unavailable (no low region)");
        return;
    }
    pool.base = base;
    pool.pages = pages;
    crate::sys::serial::print(b"[DMA] low32 pool base=");
    crate::sys::serial::print_hex(base);
    crate::sys::serial::print(b" pages=");
    crate::sys::serial::print_dec(pages as u64);
    crate::sys::serial::print(b" floor=");
    crate::sys::serial::print_dec(low32_floor(pages) as u64);
    crate::sys::serial::println(b"");
}
