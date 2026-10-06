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

//! Giving a run back to the display pool.

use super::bitmap::{clear_run, run_offset};
use super::display::DISPLAY_POOL;
use super::run_taken::run_taken;
use crate::memory::phys::PAGE_SIZE_U64;

/// `None` when `addr` is not the display pool's, so the caller frees it to
/// the general allocator; `Some(false)` when it is, but the run was refused.
pub(super) fn free(addr: u64, pages: usize) -> Option<bool> {
    let mut pool = DISPLAY_POOL.lock();
    let end = pool.base + pool.pages as u64 * PAGE_SIZE_U64;
    if pool.pages == 0 || addr < pool.base || addr >= end {
        return None;
    }
    let Some(offset) = run_offset(pool.base, pool.pages, addr, pages) else {
        return Some(false);
    };
    if !run_taken(&pool.used, offset, pages) {
        return Some(false);
    }
    clear_run(&mut pool.used, offset, pages);
    Some(true)
}
