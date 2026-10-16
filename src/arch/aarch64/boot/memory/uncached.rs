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

//! The uncached block: cut from the low DMA pool, so the pool's devices can
//! reach it, made Normal Non-cacheable in the kernel's own mapping, and
//! handed to the broker's uncached pool. What is left of the low pool is the
//! low pool. Without a block, an uncached grant for a device that does not
//! snoop is refused, never placed in memory the kernel maps cacheable.

use super::dma_window::LowPool;
use super::PAGE_SIZE;
use crate::arch::aarch64::dma::carve::split;
use crate::arch::aarch64::mmu::make_block_uncached;
use crate::hardware::broker::dma;
use crate::sys::serial::{self, Line};

pub(super) fn cut(pool: LowPool) -> LowPool {
    let Some((block, rest, rest_bytes)) = split(pool.base, pool.pages as u64 * PAGE_SIZE) else {
        serial::println(b"[DMA] no 2 MiB block in the low pool: no uncached pool");
        return pool;
    };
    if let Err(why) = make_block_uncached(block) {
        Line::new().str(b"[DMA] uncached block not made: ").str(why.as_bytes()).end();
        return pool;
    }
    dma::init_uncached_pool(block);
    LowPool { base: rest, pages: (rest_bytes / PAGE_SIZE) as usize, carved: pool.carved }
}
