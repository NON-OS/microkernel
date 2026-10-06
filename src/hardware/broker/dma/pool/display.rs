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

//! The display pool: one high contiguous run reserved at boot, so a
//! framebuffer-sized surface still finds a home once memory is fragmented.

use super::bitmap::{take_run, WORD_BITS};
use crate::hardware::broker::dma::limits::DISPLAY_FRAMEBUFFER_PAGES;
use crate::memory::phys::{alloc_contiguous, AllocFlags, PAGE_SIZE_U64};
use spin::Mutex;

const POOL_CAP_PAGES: usize = DISPLAY_FRAMEBUFFER_PAGES as usize;
const WORDS: usize = POOL_CAP_PAGES / WORD_BITS;

pub(super) struct DisplayPool {
    pub(super) base: u64,
    pub(super) pages: usize,
    pub(super) used: [u64; WORDS],
}

pub(super) static DISPLAY_POOL: Mutex<DisplayPool> =
    Mutex::new(DisplayPool { base: 0, pages: 0, used: [0; WORDS] });

pub(crate) fn init_display_pool() {
    let mut pool = DISPLAY_POOL.lock();
    if pool.pages != 0 {
        return;
    }
    for pages in [POOL_CAP_PAGES, 4096, 2048, 1024] {
        if let Some(base) = alloc_contiguous(pages, AllocFlags::DMA | AllocFlags::HIGH) {
            pool.base = base;
            pool.pages = pages;
            crate::sys::serial::print(b"[DMA] display pool base=");
            crate::sys::serial::print_hex(base);
            crate::sys::serial::print(b" pages=");
            crate::sys::serial::print_dec(pages as u64);
            crate::sys::serial::println(b"");
            return;
        }
    }
    crate::sys::serial::println(b"[DMA] display pool unavailable");
}

pub(in crate::hardware::broker::dma) fn alloc(pages: usize) -> Option<u64> {
    let mut pool = DISPLAY_POOL.lock();
    if pages == 0 || pool.pages == 0 || pages > pool.pages {
        return None;
    }
    let limit = pool.pages;
    let start = take_run(&mut pool.used, limit, pages)?;
    Some(pool.base + (start as u64 * PAGE_SIZE_U64))
}
