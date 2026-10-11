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

use super::map_user_run::map_user_run;
use crate::memory::addr::{PhysAddr, VirtAddr};
use crate::memory::paging::error::PagingResult;
use crate::memory::paging::types::PagePermissions;

// Map an MMIO range into the active user address space write-combining:
// user, read+write, no-execute, PWT alone, PAT index 1, once every CPU's PAT
// has the WC entry; strong uncached as for registers until then. Stores may
// merge and reorder until a fence, so it is only for memory without read
// side effects (a prefetchable BAR's frame buffer) or for a range the kernel
// itself maps write-combining. The hardware broker decides which, and is the
// only caller.
pub fn map_user_mmio_wc(
    virtual_addr: VirtAddr,
    physical_addr: PhysAddr,
    size: usize,
) -> PagingResult<()> {
    let base = PagePermissions::USER
        | PagePermissions::READ
        | PagePermissions::WRITE
        | PagePermissions::DEVICE;
    let permissions = if crate::arch::write_combining::write_combining_ready() {
        base | PagePermissions::WRITE_THROUGH
    } else {
        base | PagePermissions::NO_CACHE | PagePermissions::WRITE_THROUGH
    };
    map_user_run(virtual_addr, physical_addr, size, permissions)
}
