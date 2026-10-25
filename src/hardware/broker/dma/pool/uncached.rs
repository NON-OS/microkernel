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

//! The uncached pool: one 2 MiB block the architecture mapped Normal
//! Non-cacheable for the kernel too (aarch64: `mmu::make_block_uncached`).
//! Every uncached grant for a device that does not snoop comes from here
//! and nowhere else, so the capsule's Non-cacheable mapping never has a
//! cacheable twin in the kernel. Empty where the architecture needs none,
//! and then such a grant is refused, never placed in cacheable memory.

use spin::Mutex;

use super::bitmap::{clear_run, run_offset, take_run, WORD_BITS};
use super::run_taken::run_taken;
use crate::memory::phys::PAGE_SIZE_U64;

const PAGES: usize = 512;

struct Pool {
    base: u64,
    used: [u64; PAGES / WORD_BITS],
}

static POOL: Mutex<Pool> = Mutex::new(Pool { base: 0, used: [0; PAGES / WORD_BITS] });

/// Hand the pool its block, once; a later call changes nothing.
pub fn init_uncached_pool(base: u64) {
    let mut pool = POOL.lock();
    if pool.base == 0 && base != 0 {
        pool.base = base;
        drop(pool);
        crate::sys::serial::print(b"[DMA] uncached pool base=");
        crate::sys::serial::print_hex(base);
        crate::sys::serial::println(b" pages=512");
    }
}

pub(in crate::hardware::broker::dma) fn uncached_alloc(pages: usize) -> Option<u64> {
    let mut pool = POOL.lock();
    if pool.base == 0 || pages == 0 || pages > PAGES {
        return None;
    }
    let start = take_run(&mut pool.used, PAGES, pages)?;
    Some(pool.base + start as u64 * PAGE_SIZE_U64)
}

pub(super) fn uncached_owns(addr: u64) -> bool {
    let pool = POOL.lock();
    pool.base != 0 && addr >= pool.base && addr - pool.base < PAGES as u64 * PAGE_SIZE_U64
}

pub(super) fn uncached_free(addr: u64, pages: usize) -> bool {
    let mut pool = POOL.lock();
    let Some(offset) = run_offset(pool.base, PAGES, addr, pages) else {
        return false;
    };
    if !run_taken(&pool.used, offset, pages) {
        return false;
    }
    clear_run(&mut pool.used, offset, pages);
    true
}
