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
     * image, the stack or an earlier mapping.
     */
    pub fn reserve_vma(&self, length: usize, asid: u32) -> Result<VirtAddr, &'static str> {
        if length == 0 {
            return Err("EINVAL");
        }
        let pages = (length + 4095) / 4096;
        let map_flags = flags::PRESENT | flags::USER | flags::WRITABLE;
        let va = VirtAddr::new(self.reserve_unmapped(pages as u64, asid).ok_or("ENOMEM")?);
        let mut mem = self.memory_state();
        mem.vmas.push(Vma {
            start: va,
            end: VirtAddr::new(va.as_u64() + length as u64),
            flags: map_flags,
        });
        mem.resident_pages.fetch_add(pages as u64, Ordering::Relaxed);
        Ok(va)
    }
}
