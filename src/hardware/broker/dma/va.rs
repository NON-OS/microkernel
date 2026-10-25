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

//! User VA for DMA grants, from the holder's own window (K10, va_space.rs),
//! clear of the MMIO grant region and the rest of the user layout. Each
//! grant is followed by a 4 KiB guard page so an out-of-bounds access cannot
//! spill into the next, and its range goes back once its pages are unmapped.

use crate::hardware::broker::va_space::{self, Region};
use crate::memory::addr::VirtAddr;

const PAGE_SIZE: u64 = 4096;

/// `pages` of `pid`'s DMA window, or None when it is spent.
pub(super) fn reserve(pid: u32, pages: u64) -> Option<VirtAddr> {
    let bytes = pages.checked_mul(PAGE_SIZE)?.checked_add(PAGE_SIZE)?;
    va_space::reserve(pid, Region::Dma, bytes).map(VirtAddr::new)
}

/// `length` bytes at `user_va` back to `pid`'s window, already unmapped.
pub(super) fn release(pid: u32, user_va: u64, length: u64) {
    va_space::release(pid, Region::Dma, user_va, length + PAGE_SIZE);
}
