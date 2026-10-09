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

use crate::memory::addr::VirtAddr;
use alloc::vec::Vec;
use core::sync::atomic::Ordering;

use super::pcb::ProcessControlBlock;
use super::types::Vma;

impl ProcessControlBlock {
    /// Take back a span `claim_span` recorded whose mapping failed.
    pub(super) fn withdraw_span(&self, va: VirtAddr, pages: usize) {
        let mut mem = self.memory_state();
        mem.vmas.retain(|v| v.start != va);
        mem.resident_pages.fetch_sub(pages as u64, Ordering::Relaxed);
    }

    /// Remove `[addr, end)` from the VMA list and return the spans it covered.
    pub(super) fn cut_vmas(&self, addr: u64, end: u64) -> Vec<(u64, usize)> {
        let mut spans = Vec::new();
        let mut mem = self.memory_state();
        let mut i = 0usize;
        while i < mem.vmas.len() {
            let v = &mem.vmas[i];
            let vs = v.start.as_u64();
            let ve = v.end.as_u64();

            if end <= vs || addr >= ve {
                i += 1;
                continue;
            }

            let unmap_start = addr.max(vs);
            let unmap_end = end.min(ve);
            let unmap_len = (unmap_end - unmap_start) as usize;
            spans.push((unmap_start, unmap_len));
            mem.resident_pages.fetch_sub(((unmap_len + 4095) / 4096) as u64, Ordering::Relaxed);

            if unmap_start == vs && unmap_end == ve {
                mem.vmas.swap_remove(i);
                continue;
            } else if unmap_start == vs {
                mem.vmas[i].start = VirtAddr::new(unmap_end);
                i += 1;
            } else if unmap_end == ve {
                mem.vmas[i].end = VirtAddr::new(unmap_start);
                i += 1;
            } else {
                let right = Vma { start: VirtAddr::new(unmap_end), end: v.end, flags: v.flags };
                mem.vmas[i].end = VirtAddr::new(unmap_start);
                mem.vmas.push(right);
                i += 1;
            }
        }
        spans
    }
}
