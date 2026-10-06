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

use super::{map_page, unmap_page};
use crate::memory::addr::{PhysAddr, VirtAddr};
use crate::memory::paging::constants::{pages_needed, PAGE_SIZE_4K};
use crate::memory::paging::error::PagingResult;
use crate::memory::paging::types::PagePermissions;

// Map a coherent DMA buffer into the active user address space: user,
// read+write, no-execute, and never write-back, as Linux dma_alloc_coherent
// gives a device that does not snoop. Every store reaches memory without a
// cache flush, so a ring entry the driver writes is what the device reads.
//
// Uncached is PCD+PWT, PAT index 3, strong UC under the table pat::value keeps:
// not lowered by a write-combining MTRR the way UC- is (Intel SDM Vol. 3A,
// 12.12.3), and stores land in program order, which a descriptor ring whose
// owner bit is written last needs. Write-combining is PWT alone, PAT index 1,
// once pat::wc_ready says every CPU has it; stores may merge and reorder until
// a fence, so it suits a buffer the CPU fills whole before an sfence and a
// doorbell, and falls back to uncached when the PAT has no WC entry.
//
// The broker flushes the frames from the cache through the kernel's direct
// map before calling this, so no write-back line holds stale data for them.
// Caller is the hardware broker only.
pub fn map_user_dma_coherent(
    virtual_addr: VirtAddr,
    physical_addr: PhysAddr,
    size: usize,
    write_combining: bool,
) -> PagingResult<()> {
    let base = PagePermissions::USER | PagePermissions::READ | PagePermissions::WRITE;
    let permissions = if write_combining && crate::arch::write_combining::write_combining_ready() {
        base | PagePermissions::WRITE_THROUGH
    } else {
        base | PagePermissions::NO_CACHE | PagePermissions::WRITE_THROUGH
    };
    let pages = pages_needed(size);
    for i in 0..pages {
        let va = VirtAddr::new(virtual_addr.as_u64() + (i * PAGE_SIZE_4K) as u64);
        let pa = PhysAddr::new(physical_addr.as_u64() + (i * PAGE_SIZE_4K) as u64);
        if let Err(e) = map_page(va, pa, permissions) {
            for j in 0..i {
                let rb = VirtAddr::new(virtual_addr.as_u64() + (j * PAGE_SIZE_4K) as u64);
                let _ = unmap_page(rb);
            }
            return Err(e);
        }
    }
    Ok(())
}
