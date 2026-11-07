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

use crate::arch::paging::descriptor::flags;
use crate::memory::addr::VirtAddr;
use core::sync::atomic::Ordering;

use super::pcb::ProcessControlBlock;
use super::types::Vma;

impl ProcessControlBlock {
    /*
     * Reserve a free user-half VA window and record it as a VMA. PTE
     * installation is the caller's responsibility; this is used by the
     * surface registry to land a foreign frame list in this AS. The window
     * comes from the same allocator as anonymous mmap and is checked
     * against the live page tables of `asid`, so it cannot land on the ELF
     * image, the stack or an earlier mapping. A page either side stays
     * unmapped, so an overrun of the view, or into it, faults.
     */
    pub fn reserve_vma(&self, length: usize, asid: u32) -> Result<VirtAddr, &'static str> {
        if length == 0 {
            return Err("EINVAL");
        }
        let pages = (length + 4095) / 4096;
        let map_flags = flags::PRESENT | flags::USER | flags::WRITABLE;
        let va = VirtAddr::new(self.reserve_unmapped_guarded(pages as u64, asid).ok_or("ENOMEM")?);
        let mut mem = self.memory_state();
        mem.vmas.push(Vma {
            start: va,
            end: VirtAddr::new(va.as_u64() + length as u64),
            flags: map_flags,
        });
        mem.resident_pages.fetch_add(pages as u64, Ordering::Relaxed);
        Ok(va)
    }

    /*
     * Give back the window `reserve_vma` took for a surface view at `start`,
     * `pages` long, once the view is down: its VMA, its resident count and
     * its addresses with their guards. Only a window this process recorded
     * as a VMA of exactly that span and its allocator holds as a guarded
     * range is given back, so no mapping of the process's own and no other
     * thread's window is touched. False when nothing was given back.
     */
    pub fn give_back_view(&self, start: VirtAddr, pages: u64) -> bool {
        let Some(end) = pages.checked_mul(4096).and_then(|n| start.as_u64().checked_add(n)) else {
            return false;
        };
        /*
         * The VMA comes out first and goes back if the allocator refuses:
         * the two locks are never held together, as in `reserve_vma`, and
         * while the allocator holds the window no attach can be given it.
         */
        let vma = {
            let mut mem = self.memory_state();
            let at = mem.vmas.iter().position(|v| v.start == start && v.end.as_u64() == end);
            match at {
                Some(at) => mem.vmas.remove(at),
                None => return false,
            }
        };
        if !self.mmap_va.lock().release_guarded(start.as_u64(), pages) {
            self.memory_state().vmas.push(vma);
            return false;
        }
        let mem = self.memory_state();
        let _ = mem
            .resident_pages
            .fetch_update(Ordering::Relaxed, Ordering::Relaxed, |n| Some(n.saturating_sub(pages)));
        true
    }

    /* Give back a window from `reserve_vma` whose mapping failed. */
    pub fn unreserve_vma(&self, start: VirtAddr, length: usize) {
        let pages = length.div_ceil(4096) as u64;
        let mut mem = self.memory_state();
        if let Some(at) = mem.vmas.iter().position(|v| v.start == start) {
            mem.vmas.remove(at);
            mem.resident_pages.fetch_sub(pages, Ordering::Relaxed);
        }
        drop(mem);
        // And its addresses, which `reserve_unmapped` took.
        let _ = self.mmap_va.lock().release(start.as_u64(), pages);
    }
}
