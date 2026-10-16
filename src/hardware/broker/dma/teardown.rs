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

//! One grant's revocation, in the order `release.rs` describes.

use super::pool;
use super::types::DmaGrant;
use crate::memory::addr::VirtAddr;
use crate::memory::phys::free_contiguous;

const PAGE_SIZE: u64 = 4096;

pub(super) fn teardown(g: &DmaGrant, unmap_pages: bool) {
    if withdraw(g, unmap_pages) {
        release_frames(g);
    }
}

/// The grant out of its holder's pages and its device's domain, the IOTLB
/// invalidation waited on. False when the domain will not give it up: the
/// frames are then leaked, never reused.
pub(super) fn withdraw(g: &DmaGrant, unmap_pages: bool) -> bool {
    // From the tables the holder runs in, from any context: a holder
    // whose tables outlive it, a thread's or ones handed to an heir, must
    // not keep a page onto frames freed below.
    let unmapped = super::super::holder_space::unmap_from_holder(g.pid, g.user_va, g.length);
    if !unmapped && unmap_pages {
        let _ = crate::memory::paging::unmap_user_dma(VirtAddr::new(g.user_va), g.length as usize);
    }
    if !super::super::confine::unmap(g.pid, g.device_addr, g.length, g.confined) {
        crate::sys::serial::println(
            b"[DMA] grant still reachable by its device; frames quarantined",
        );
        return false;
    }
    if !g.confined {
        crate::memory::iommu::note_unconfined_released(1);
    }
    true
}

/// A withdrawn grant's frames back to the allocator, or kept for a surface
/// another process still maps, which frees them once nobody does.
pub(super) fn release_frames(g: &DmaGrant) {
    if crate::kernel_core::surface_registry::pin::dma_hold::hold_dma(
        g.pid,
        g.physical_start,
        g.length,
    ) {
        return;
    }
    free_frames(g.physical_start, g.length);
}

/// Scrub a grant's frames and return them to the pool they came from.
pub(crate) fn free_frames(start: u64, len: u64) {
    super::scrub::scrub(start, len);
    let pages = (len / PAGE_SIZE) as usize;
    if !pool::give_back(start, pages) {
        let _ = free_contiguous(start, pages);
    }
}
