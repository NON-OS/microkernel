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

//! Pages with an unmapped page below them, for stacks.
//!
//! A stack grows down. Placed straight after another block of this
//! allocator, a stack that runs past its bottom writes into that block, and
//! nothing notices until whatever lives there is used: two kernel stacks side
//! by side meant one process's overrun became another's corrupt return
//! address. One page of address space below the
//! block is reserved and never mapped, so the first write past the bottom
//! faults on the spot, on the CPU and stack that made it. The target's
//! inline stack probes touch every page of a large frame in order, so no
//! frame steps over the guard.

use super::super::super::constants::PAGE_SIZE;
use super::super::super::error::{BuddyAllocError, BuddyAllocResult};
use super::mapping::map_page;
use super::release::release_in;
use super::stats::VMAP_ALLOCATOR;
use crate::memory::addr::VirtAddr;
use crate::memory::frame_alloc;
use core::ptr;

/// `count` zeroed, mapped pages with one unmapped guard page below them. The
/// address is the lowest mapped page.
pub fn allocate_guarded(count: usize) -> BuddyAllocResult<VirtAddr> {
    if count == 0 {
        return Err(BuddyAllocError::InvalidPageCount);
    }
    let total = count.checked_add(1).ok_or(BuddyAllocError::Overflow)?;
    let size = total.checked_mul(PAGE_SIZE).ok_or(BuddyAllocError::Overflow)?;
    let range = VMAP_ALLOCATOR.lock().allocate_range(size, PAGE_SIZE)?;
    let base = VirtAddr::new(range.as_u64() + PAGE_SIZE as u64);
    for i in 0..count {
        let page = VirtAddr::new(base.as_u64() + (i * PAGE_SIZE) as u64);
        let mapped = frame_alloc::allocate_frame()
            .ok_or(BuddyAllocError::FrameAllocationFailed)
            .and_then(|phys| match map_page(page, phys) {
                Ok(()) => Ok(()),
                Err(e) => {
                    let _ = frame_alloc::deallocate_frame(phys);
                    Err(e)
                }
            });
        if let Err(e) = mapped {
            give_back(range, base, i);
            return Err(e);
        }
    }
    // SAFETY: the `count` pages at `base` were mapped writable above and
    // nobody else has their address yet.
    unsafe { ptr::write_bytes(base.as_mut_ptr::<u8>(), 0, count * PAGE_SIZE) };
    Ok(base)
}

/// Scrub and free what `allocate_guarded(count)` returned at `base`, the
/// guard's address space with it.
pub fn free_guarded(base: VirtAddr, count: usize) -> BuddyAllocResult<()> {
    if count == 0 {
        return Err(BuddyAllocError::InvalidPageCount);
    }
    let range = base.as_u64().checked_sub(PAGE_SIZE as u64).ok_or(BuddyAllocError::InvalidAddress)?;
    let size = count.checked_mul(PAGE_SIZE).ok_or(BuddyAllocError::Overflow)?;
    // SAFETY: the caller hands back pages `allocate_guarded` mapped and no
    // CPU runs on any more; what a stack held is not left in freed frames.
    unsafe { ptr::write_bytes(base.as_mut_ptr::<u8>(), 0, size) };
    release_in(VirtAddr::new(range), base, count, true)
}

/// Undo an allocation that mapped only its first `mapped` pages.
fn give_back(range: VirtAddr, base: VirtAddr, mapped: usize) {
    if mapped == 0 {
        let _ = VMAP_ALLOCATOR.lock().deallocate_range(range);
    } else {
        let _ = release_in(range, base, mapped, true);
    }
}
