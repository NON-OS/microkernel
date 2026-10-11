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
use crate::kernel_core::surface_registry::pin::busy;
use crate::kernel_core::surface_registry::pin::held::Held;
use crate::memory::paging::unmap_page;
use crate::memory::VirtAddr;

use super::super::consts::PAGE_SIZE;

/*
 * Unmap `pages` pages from `addr` for `pid` and free their frames. Frames of
 * a surface another process still maps are not ours to free: freeing them
 * let the allocator reuse a frame that process kept writing pixels into.
 * They go to the surface registry instead. The range is busy from before
 * the surfaces are read until they are settled (`pin::busy`), so a sibling
 * thread's register of these pages meanwhile is refused.
 */
pub(in super::super) fn unmap_range(pid: u32, addr: u64, pages: u64) {
    let len = pages * PAGE_SIZE as u64;
    let _busy = busy::begin(addr, len);
    let mut held = Held::collect(pid, addr, len);
    for i in 0..pages as usize {
        /*
         * Up to a gigabyte unmapped with interrupts masked. Answer TLB
         * shootdowns between pages; each page's frame is freed before the
         * next serve point, so no translation is carried across one.
         */
        crate::smp::serve_shootdowns();
        let va = VirtAddr::new(addr + (i * PAGE_SIZE) as u64);
        if let Ok(phys) = unmap_page(va) {
            if held.holds(va.as_u64(), phys) {
                continue;
            }
            if crate::memory::frame_alloc::deallocate_frame(phys).is_err() {
                crate::sys::serial::println(b"[MUNMAP] frame_release_failed");
            }
        }
    }
    held.settle();
}
