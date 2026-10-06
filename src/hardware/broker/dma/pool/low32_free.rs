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

//! Giving a run back to the low pool.

use super::bitmap::{clear_run, run_offset};
use super::low32::LOW32_POOL;
use super::run_taken::run_taken;
use crate::memory::phys::PAGE_SIZE_U64;

/// Whether `addr` was handed out by the low pool, so a free is routed here.
pub(super) fn low32_owns(addr: u64) -> bool {
    let pool = LOW32_POOL.lock();
    pool.pages != 0 && addr >= pool.base && addr < pool.base + (pool.pages as u64 * PAGE_SIZE_U64)
}

pub(super) fn low32_free(addr: u64, pages: usize) -> bool {
    let mut pool = LOW32_POOL.lock();
    let Some(offset) = run_offset(pool.base, pool.pages, addr, pages) else {
        return false;
    };
    if !run_taken(&pool.used, offset, pages) {
        return false;
    }
    clear_run(&mut pool.used, offset, pages);
    true
}
