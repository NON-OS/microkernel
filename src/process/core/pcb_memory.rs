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
use crate::memory::addr::{PhysAddr, VirtAddr};

use super::pcb::ProcessControlBlock;
use super::types::align_up;

impl ProcessControlBlock {
    pub fn mmap(
        &self,
        hint: Option<VirtAddr>,
        length: usize,
        flags: u64,
    ) -> Result<VirtAddr, &'static str> {
        if length == 0 {
            return Err("EINVAL");
        }
        let pages = (length + 4095) / 4096;
        let map_flags =
            flags::PRESENT | flags::USER | (flags & (flags::WRITABLE | flags::NO_EXECUTE));

        /*
         * The span is claimed as a VMA before any page is mapped, so a second
         * caller cannot pick the same addresses while the lock is dropped for
         * the mapping, and withdrawn again if the mapping fails.
         */
        let va = self.claim_span(hint, length, pages, map_flags)?;

        let mut allocated_pages: usize = 0;

        let result = (|| -> Result<(), &'static str> {
            for i in 0..pages {
                /*
                 * Masked for the whole span; answer shootdowns once per page.
                 * No translation is carried across this point.
                 */
                crate::smp::serve_shootdowns();
                let page_va = VirtAddr::new(va.as_u64() + (i as u64) * 4096);
                let phys = allocate_physical_page().ok_or("ENOMEM")?;
                map_page_to_phys(page_va, phys, map_flags).map_err(|_| "EIO")?;
                allocated_pages += 1;
                // SAFETY: Page was just mapped with valid physical memory backing.
                unsafe {
                    core::ptr::write_bytes(page_va.as_u64() as *mut u8, 0, 4096);
                }
            }
            Ok(())
        })();

        if result.is_err() {
            for i in 0..allocated_pages {
                let page_va = VirtAddr::new(va.as_u64() + (i as u64) * 4096);
                let _ = unmap_range(page_va, 4096);
            }
            self.withdraw_span(va, pages);
            return result.map(|_| va);
        }

        let mut mem = self.memory_state();
        mem.next_va = align_up(va.as_u64() + length as u64, 0x1000);
        Ok(va)
    }

    pub fn munmap(&self, addr: VirtAddr, length: usize) -> Result<(), &'static str> {
        if length == 0 || (addr.as_u64() & 0xFFF) != 0 {
            return Err("EINVAL");
        }
        let end = addr.as_u64().checked_add(length as u64).ok_or("EINVAL")?;

        /*
         * The VMAs are cut under the lock and the pages are unmapped after it
         * is dropped. `unmap_range` waits for every CPU running this address
         * space to acknowledge a shootdown, and one of them may be spinning
         * for this very lock.
         */
        let mut outcome = Ok(());
        for (start, len) in self.cut_vmas(addr.as_u64(), end) {
            if unmap_range(VirtAddr::new(start), len).is_err() {
                outcome = Err("EIO");
            }
        }
        outcome
    }
}

fn allocate_physical_page() -> Option<PhysAddr> {
    crate::memory::phys::alloc(crate::memory::phys::AllocFlags::empty())
        .map(|f| crate::memory::addr::PhysAddr::new(f.0))
}

fn map_page_to_phys(page_va: VirtAddr, phys: PhysAddr, _flags: u64) -> Result<(), ()> {
    use crate::memory::paging::types::PagePermissions;
    let perms = PagePermissions::READ | PagePermissions::WRITE;
    crate::memory::paging::manager::map_page(page_va, phys, perms).map_err(|_| ())
}

fn unmap_range(addr: VirtAddr, len: usize) -> Result<(), ()> {
    crate::memory::paging::manager::unmap_range(addr, len).map_err(|_| ())
}
