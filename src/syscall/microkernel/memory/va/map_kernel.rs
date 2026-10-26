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
use crate::memory::addr::PhysAddr;
use crate::memory::paging::{map_page, PagePermissions};
use crate::memory::VirtAddr;

use super::super::consts::PAGE_SIZE;
use super::map_loop::MapOps;
use super::rollback_mapped_pages;

/// The kernel's steps for `map_loop`.
pub(super) struct Kernel {
    pub(super) base: u64,
    pub(super) perms: PagePermissions,
}

impl MapOps for Kernel {
    type Frame = PhysAddr;

    fn serve_shootdowns(&mut self) {
        crate::smp::serve_shootdowns();
    }

    fn window(&mut self) {
        if crate::arch::cpu::interrupts_enabled() {
            return;
        }
        crate::arch::enable_interrupts();
        // On x86_64, STI keeps interrupts masked for one more instruction;
        // this pause is that instruction, so a pending interrupt is taken
        // after it, before the mask closes. A bare `sti; cli` would let
        // nothing in.
        core::hint::spin_loop();
        crate::arch::disable_interrupts();
    }

    fn allocate(&mut self) -> Option<PhysAddr> {
        crate::memory::frame_alloc::allocate_frame()
    }

    fn zero(&mut self, frame: PhysAddr) {
        crate::memory::frame_alloc::zero_frame(frame);
    }

    fn map(&mut self, i: usize, frame: PhysAddr) -> bool {
        let va = VirtAddr::new(self.base + (i * PAGE_SIZE) as u64);
        map_page(va, frame, self.perms).is_ok()
    }

    fn free(&mut self, frame: PhysAddr) {
        if crate::memory::frame_alloc::deallocate_frame(frame).is_err() {
            crate::sys::serial::println(b"[MMAP] frame_release_failed");
        }
    }

    fn rollback(&mut self, mapped: usize) {
        rollback_mapped_pages(self.base, mapped);
    }
}
