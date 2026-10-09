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
    if unmap_pages {
        let _ = crate::memory::paging::unmap_user_dma(VirtAddr::new(g.user_va), g.length as usize);
    }
    // The device loses the grant before anyone else can gain the frames. If
    // its domain will not give it up, the frames are leaked, never reused.
    if !super::super::confine::unmap(g.pid, g.device_addr, g.length, g.confined) {
        crate::sys::serial::println(
            b"[DMA] grant still reachable by its device; frames quarantined",
        );
        return;
    }
    super::scrub::scrub(g.physical_start, g.length);
    let pages = (g.length / PAGE_SIZE) as usize;
    if !pool::give_back(g.physical_start, pages) {
        let _ = free_contiguous(g.physical_start, pages);
    }
    if !g.confined {
        crate::memory::iommu::note_unconfined_released(1);
    }
}
