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
use crate::dma::{DmaPool, DmaRegion};
use crate::error::{XhciError, XhciResult};
use crate::regs::op::MAX_PAGE_BYTES;
use alloc::vec::Vec;
const SCRATCHPAD_PTR_BYTES: u64 = 8;
const POOL_PAGE_BYTES: u64 = 4096;
/// The scratchpad buffers HCSPARAMS2 asks for (xHCI 1.2 section 4.20): an
/// array of `count` pointers, DCBAA entry 0, each to one zeroed page of the
/// controller's own page size (PAGESIZE), aligned to that size. The pool
/// aligns to 4 KiB; a larger page is carved out of a grant twice its size.
pub enum Scratchpads {
    None,
    Allocated { array: DmaRegion, pages: Vec<DmaRegion> },
}
impl Scratchpads {
    pub fn allocate(pool: &DmaPool, count: u32, page_bytes: u64) -> XhciResult<Self> {
        if count == 0 {
            return Ok(Scratchpads::None);
        }
        if !page_bytes.is_power_of_two()
            || !(POOL_PAGE_BYTES..=MAX_PAGE_BYTES).contains(&page_bytes)
        {
            return Err(XhciError::ControllerUnsupported);
        }
        let array_bytes = (count as u64) * SCRATCHPAD_PTR_BYTES;
        let array = pool.alloc(array_bytes)?;
        array.zero();
        let grant_bytes = if page_bytes == POOL_PAGE_BYTES { page_bytes } else { 2 * page_bytes };
        let mut pages: Vec<DmaRegion> = Vec::with_capacity(count as usize);
        let array_va = array.as_mut_ptr::<u64>();
        for i in 0..count as usize {
            let page = pool.alloc(grant_bytes)?;
            page.zero();
            let phys = aligned_page(page.phys(), page_bytes);
            unsafe {
                core::ptr::write_volatile(array_va.add(i), phys);
            }
            pages.push(page);
        }
        Ok(Scratchpads::Allocated { array, pages })
    }
    pub fn array_phys(&self) -> u64 {
        match self {
            Scratchpads::None => 0,
            Scratchpads::Allocated { array, .. } => array.phys(),
        }
    }
    pub fn page_count(&self) -> u32 {
        match self {
            Scratchpads::None => 0,
            Scratchpads::Allocated { pages, .. } => pages.len() as u32,
        }
    }
}

/// The first `page_bytes`-aligned address at or after `phys`. A grant of
/// twice `page_bytes` always holds one whole aligned page from there.
pub fn aligned_page(phys: u64, page_bytes: u64) -> u64 {
    (phys + page_bytes - 1) & !(page_bytes - 1)
}
