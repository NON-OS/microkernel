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

//! Reaching and creating page tables, for every walker in this module.

use crate::arch::paging::descriptor;
use crate::memory::addr::PhysAddr;
use crate::memory::paging::constants::{PAGE_SIZE_4K, PAGE_TABLE_ENTRIES};
use crate::memory::paging::error::{PagingError, PagingResult};
use crate::memory::{frame_alloc, layout};

/// The directmap view of the table at `pa`.
pub(super) fn table_at(pa: PhysAddr) -> *mut [u64; PAGE_TABLE_ENTRIES] {
    (layout::DIRECTMAP_BASE + pa.as_u64()) as *mut [u64; PAGE_TABLE_ENTRIES]
}

/// Back an absent intermediate entry with a fresh zeroed table.
pub(super) fn alloc_table(entry: &mut u64) -> PagingResult<()> {
    let new = frame_alloc::allocate_frame().ok_or(PagingError::FrameAllocationFailed)?;
    // The intermediate levels impose no restriction; the leaf decides.
    *entry = descriptor::table(new.as_u64(), true);
    /*
     * SAFETY: eK@nonos.systems - the frame came from the allocator a
     * moment ago, is 4 KiB, and is reachable through the directmap.
     */
    unsafe {
        core::ptr::write_bytes((layout::DIRECTMAP_BASE + new.as_u64()) as *mut u8, 0, PAGE_SIZE_4K);
    }
    Ok(())
}
