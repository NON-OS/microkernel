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

use crate::kernel_core::surface_registry::types::SurfaceHandle;
use crate::memory::addr::VirtAddr;
use crate::memory::paging::manager::api::{translate_in_asid, unmap_page_in_asid};
use crate::memory::paging::types::PagePermissions;

use super::frames_of::frames_of;

/*
 * `pid` drops its attach of `handle` at `base`, under the gate its caller
 * holds. When it does not own the surface, its view of the owner's frames
 * goes with the record, so no PTE outlives the record that `reclaim` checks
 * before freeing them. Only pages that still map the surface frame are
 * removed, and none of them is freed. False when its tables still exist but
 * their address space cannot be found: the record must then stay, or the
 * frames are freed under a view.
 */
pub(super) fn unmap_receiver(pid: u32, handle: SurfaceHandle, base: u64) -> bool {
    let Some((owner, frames)) = frames_of(handle) else { return true };
    if owner == pid {
        return true;
    }
    let space = super::super::space::space_of(pid);
    let Some(asid) = space else { return super::super::space::tables_of(pid).is_none() };
    let perms = PagePermissions::user_rw();
    for (i, frame) in frames.iter().enumerate() {
        let va = VirtAddr::new(base.wrapping_add((i as u64) * 4096));
        let mapped = translate_in_asid(asid, va).map(|pa| pa.as_u64() & !0xFFF);
        if mapped == Some(frame.as_u64()) {
            let _ = unmap_page_in_asid(asid, va, perms);
        }
    }
    true
}
