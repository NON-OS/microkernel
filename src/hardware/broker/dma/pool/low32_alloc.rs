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

//! Taking runs from the low pool.

use super::bitmap::{count_used, take_run};
use super::low32::{LOW32_MAX_PAGES, LOW32_POOL};
use super::sizing::low32_floor;
use crate::memory::phys::PAGE_SIZE_U64;

/// Allocate `pages` from the low pool, or `None` if it is too large for the pool
/// policy, the pool is uninitialised, or it is full. `needs_32bit` is a map
/// whose device can only name a 32-bit address: it may take the reserved floor,
/// which every other map leaves free.
pub(in crate::hardware::broker::dma) fn low32_alloc(
    pages: usize,
    needs_32bit: bool,
) -> Option<u64> {
    if pages == 0 || pages > LOW32_MAX_PAGES {
        return None;
    }
    let mut pool = LOW32_POOL.lock();
    if pool.pages == 0 || pages > pool.pages {
        return None;
    }
    let free = pool.pages.saturating_sub(count_used(&pool.used));
    let floor = low32_floor(pool.pages);
    let placement = crate::hardware::broker::dma::placement::low32_may_take;
    if !placement(free, pages, pool.pages, floor, needs_32bit) {
        return None;
    }
    let limit = pool.pages;
    let start = take_run(&mut pool.used, limit, pages)?;
    let (used, total) = (count_used(&pool.used), pool.pages);
    let base = pool.base;
    drop(pool);
    // Said after the lock is dropped: the serial line is slow.
    super::pressure::note(used, total);
    Some(base + (start as u64 * PAGE_SIZE_U64))
}
