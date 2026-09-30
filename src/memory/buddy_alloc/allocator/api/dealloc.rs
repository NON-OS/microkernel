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

//! Giving vmap pages back: unmap, then free the frames, then the range.
//!
//! In that order and no other. A range or frame given back while some cpu
//! still caches its old translation could be handed out and reached through
//! the stale entry; and installing over an absent entry skips the remote
//! flush on the strength of this order (see the paging manager's
//! `PendingFlush::after_install`).

use super::super::super::constants::PAGE_SIZE;
use super::super::super::error::{BuddyAllocError, BuddyAllocResult};
use super::stats::VMAP_ALLOCATOR;
use crate::memory::addr::{PhysAddr, VirtAddr};
use crate::memory::frame_alloc;
use crate::memory::paging::manager;

/// Pages unmapped under one shootdown round. A kernel stack is eight; the
/// round's own range flush turns anything past 32 pages into a full flush.
const BATCH: usize = 32;

pub fn free_pages(addr: VirtAddr, count: usize) -> BuddyAllocResult<()> {
    release(addr, count, true)
}

pub fn free_aligned(addr: VirtAddr, size: usize) -> BuddyAllocResult<()> {
    let page_count = size.checked_add(PAGE_SIZE - 1).ok_or(BuddyAllocError::Overflow)? / PAGE_SIZE;
    free_pages(addr, page_count)
}

pub fn deallocate_pages(addr: VirtAddr, count: usize) -> BuddyAllocResult<()> {
    release(addr, count, false)
}

/*
 * One shootdown round per batch instead of one per page. Kernel stacks are
 * released from the timer tick with interrupts masked, and each round waits
 * for every cpu; eight rounds for one stack was eight such waits. The frames
 * are read before the unmap and freed only after it, whose flush has been
 * acknowledged by the time it returns.
 */
fn release(addr: VirtAddr, count: usize, free_frames: bool) -> BuddyAllocResult<()> {
    if count == 0 {
        return Err(BuddyAllocError::InvalidPageCount);
    }
    let mut done = 0;
    while done < count {
        let n = (count - done).min(BATCH);
        let offset = done.checked_mul(PAGE_SIZE).ok_or(BuddyAllocError::Overflow)?;
        let base = VirtAddr::new(addr.as_u64() + offset as u64);
        let mut frames = [PhysAddr::new(0); BATCH];
        for (i, frame) in frames.iter_mut().enumerate().take(n) {
            let va = VirtAddr::new(base.as_u64() + (i * PAGE_SIZE) as u64);
            *frame = manager::translate_address(va).ok_or(BuddyAllocError::TranslationFailed)?;
        }
        manager::unmap_range(base, n * PAGE_SIZE).map_err(|_| BuddyAllocError::UnmapFailed)?;
        if free_frames {
            for frame in &frames[..n] {
                let _ = frame_alloc::deallocate_frame(*frame);
            }
        }
        done += n;
    }
    VMAP_ALLOCATOR.lock().deallocate_range(addr)
}

pub fn deallocate_aligned(addr: VirtAddr, size: usize) -> BuddyAllocResult<()> {
    let page_count = size.checked_add(PAGE_SIZE - 1).ok_or(BuddyAllocError::Overflow)? / PAGE_SIZE;
    deallocate_pages(addr, page_count)
}
