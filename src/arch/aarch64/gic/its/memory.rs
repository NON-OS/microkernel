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

//! Physically contiguous, zeroed memory the ITS or a redistributor reads:
//! tables, the command queue, translation tables.

use crate::memory::addr::PhysAddr;
use crate::memory::phys::{alloc_contiguous, free_contiguous, AllocFlags};
use crate::memory::unified::phys_to_virt;

const FRAME: u64 = 0x1000;

#[derive(Clone, Copy)]
pub(super) struct Region {
    pub(super) phys: u64,
    pub(super) bytes: u64,
}

/// `bytes` in whole frames at a multiple of `align`, a power of two of a
/// frame or more. The allocator aligns to a frame only, so this takes
/// `align` less a frame more and gives the slack either side back.
pub(super) fn alloc(bytes: u64, align: u64) -> Option<Region> {
    let frames = bytes.div_ceil(FRAME);
    let slack = align / FRAME - 1;
    let base = alloc_contiguous((frames + slack) as usize, AllocFlags::ZERO)?;
    let start = base.next_multiple_of(align);
    let head = (start - base) / FRAME;
    let _ = free_contiguous(base, head as usize);
    let _ = free_contiguous(start + frames * FRAME, (slack - head) as usize);
    let region = Region { phys: start, bytes: frames * FRAME };
    let Some(view) = view(&region) else {
        let _ = free_contiguous(region.phys, frames as usize);
        return None;
    };
    // Zeroed and cleaned whatever the caller learns of coherency later: a
    // zero device table entry is invalid, a zero LPI is disabled.
    view.fill(0);
    clean(&region, 0, region.bytes);
    Some(region)
}

pub(super) fn view(region: &Region) -> Option<&'static mut [u8]> {
    let virt = phys_to_virt(PhysAddr::new(region.phys))?.as_u64();
    // SAFETY: frames `alloc` took for the caller alone, reached through the
    // direct map; only the ITS driver writes them, under its lock.
    Some(unsafe { core::slice::from_raw_parts_mut(virt as *mut u8, region.bytes as usize) })
}

/// Write `[offset, offset + len)` out for a reader that does not snoop.
pub(super) fn clean(region: &Region, offset: u64, len: u64) {
    if let Some(virt) = phys_to_virt(PhysAddr::new(region.phys + offset)) {
        crate::arch::cache_flush::flush_and_fence(virt.as_u64(), len);
    }
}

/// Scrubbed first: a translation table names physical memory.
pub(super) fn free(region: Region) {
    if let Some(view) = view(&region) {
        view.fill(0);
    }
    let _ = free_contiguous(region.phys, (region.bytes / FRAME) as usize);
}
