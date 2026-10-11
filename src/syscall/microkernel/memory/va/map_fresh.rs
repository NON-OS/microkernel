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
use crate::memory::paging::{map_page, PagePermissions};
use crate::memory::VirtAddr;

use super::super::consts::PAGE_SIZE;
use super::rollback_mapped_pages;

/*
 * Back `pages` pages from `base` with fresh frames, zeroed before each is
 * mapped. On failure the pages this call mapped are unmapped and freed, and
 * false is returned. The range is busy throughout (`pin::busy`): a sibling
 * thread could otherwise register pages the rollback then frees.
 */
pub(in super::super) fn map_fresh(base: u64, pages: u64, perms: PagePermissions) -> bool {
    let len = pages * PAGE_SIZE as u64;
    let _busy = crate::kernel_core::surface_registry::pin::busy::begin(base, len);
    for i in 0..pages as usize {
        /*
         * Up to a gigabyte mapped with interrupts masked: serve TLB shootdowns.
         */
        crate::smp::serve_shootdowns();
        let va = VirtAddr::new(base + (i * PAGE_SIZE) as u64);
        let Some(frame) = crate::memory::frame_alloc::allocate_frame() else {
            rollback_mapped_pages(base, i);
            return false;
        };
        // Zeroed before it is mapped: mapped first, a sibling thread could
        // read the frame's last owner's data through it until the zeroing.
        crate::memory::frame_alloc::zero_frame(frame);
        if map_page(va, frame, perms).is_err() {
            if crate::memory::frame_alloc::deallocate_frame(frame).is_err() {
                crate::sys::serial::println(b"[MMAP] frame_release_failed");
            }
            rollback_mapped_pages(base, i);
            return false;
        }
    }
    true
}
