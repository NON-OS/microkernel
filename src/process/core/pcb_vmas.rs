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

//! The VMA list of a process: the lock that guards it, and the claims
//! `mmap` makes in it. `pcb_vma_cut` takes spans out again.

use crate::memory::addr::VirtAddr;
use core::sync::atomic::Ordering;
use spin::MutexGuard;

use super::pcb::ProcessControlBlock;
use super::types::{align_up, overlaps, MemoryState, Vma};

impl ProcessControlBlock {
    /// The address-space bookkeeping, taken without going deaf to TLB
    /// shootdowns.
    ///
    /// Every system call runs with interrupts masked, so a CPU spinning here
    /// with `lock()` could not acknowledge a shootdown aimed at the address
    /// space it is running, and a holder waiting on that acknowledgement
    /// would never release. Nothing may change a page table while holding
    /// this guard either; `mmap` and `munmap` map and unmap with it
    /// dropped.
    pub fn memory_state(&self) -> MutexGuard<'_, MemoryState> {
        crate::smp::lock_responsive(&self.memory)
    }

    /// Pick `length` free bytes at `hint` or above `next_va`, and record them
    /// as a VMA of `pages` pages with `map_flags`.
    pub(super) fn claim_span(
        &self,
        hint: Option<VirtAddr>,
        length: usize,
        pages: usize,
        map_flags: u64,
    ) -> Result<VirtAddr, &'static str> {
        let mut mem = self.memory_state();
        let va = match hint {
            Some(h) if (h.as_u64() & 0xFFF) == 0 && !overlaps(&mem.vmas, h, length) => h,
            _ => {
                let mut candidate = align_up(mem.next_va, 0x1000);
                loop {
                    if candidate > 0x0000_FFFF_FFFF_F000 {
                        return Err("ENOMEM");
                    }
                    let cand = VirtAddr::new(candidate);
                    if !overlaps(&mem.vmas, cand, length) {
                        break cand;
                    }
                    candidate = align_up(candidate + length as u64, 0x1000);
                }
            }
        };
        mem.vmas.push(Vma {
            start: va,
            end: VirtAddr::new(va.as_u64() + length as u64),
            flags: map_flags,
        });
        mem.resident_pages.fetch_add(pages as u64, Ordering::Relaxed);
        Ok(va)
    }
}
