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

//! The pages of a receiver's view, mapped whole or taken down again.

use crate::memory::addr::{PhysAddr, VirtAddr};
use crate::memory::paging::manager::api::{map_page_in_asid, unmap_page_in_asid};
use crate::memory::paging::types::PagePermissions;

/*
 * Map `frames` with `perms` at consecutive pages from `base` in `asid`: user
 * read-only unless the receiver was named to write. When one fails,
 * the pages already mapped come down again: left with no attach record,
 * they outlived the frames (L49). Err(true) if one would not.
 */
pub(super) fn map_pages(
    asid: u32,
    base: VirtAddr,
    frames: &[PhysAddr],
    perms: PagePermissions,
) -> Result<(), bool> {
    let page = |i: usize| VirtAddr::new(base.as_u64() + (i as u64) * 4096);
    for (i, frame) in frames.iter().enumerate() {
        if map_page_in_asid(asid, page(i), *frame, perms).is_err() {
            let stuck = (0..i).filter(|&j| unmap_page_in_asid(asid, page(j), perms).is_err());
            return Err(stuck.count() > 0);
        }
    }
    Ok(())
}
