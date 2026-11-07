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

use super::view_rule::{window_back, Window};

/*
 * `pid` drops its attach of `handle` at `base`, under the gate its caller
 * holds. When it does not own the surface, its view of the project's frames
 * goes with the record, so no PTE outlives the record that `reclaim` checks
 * before freeing them. Only pages that still map the surface frame are
 * removed, and none of them is freed. `Lost` when its tables still exist but
 * their address space cannot be found: the record must then stay, or the
 * frames are freed under a view. `Free(pages)` when the view's whole window
 * is unmapped now, so the window its attach reserved can be given back.
 */
pub(super) fn unmap_receiver(pid: u32, handle: SurfaceHandle, base: u64) -> Window {
    let Some((owner, frames)) = frames_of(handle) else { return Window::Kept };
    if owner == pid {
        return window_back(true, frames.len() as u64, 0);
    }
    let space = super::super::space::space_of(pid);
    let Some(asid) = space else {
        return if super::super::space::tables_of(pid).is_none() {
            Window::Kept
        } else {
            Window::Lost
        };
    };
    let perms = PagePermissions::user_rw();
    let at = |i: usize| VirtAddr::new(base.wrapping_add((i as u64) * 4096));
    for (i, frame) in frames.iter().enumerate() {
        let mapped = translate_in_asid(asid, at(i)).map(|pa| pa.as_u64() & !0xFFF);
        if mapped == Some(frame.as_u64()) {
            let _ = unmap_page_in_asid(asid, at(i), perms);
        }
    }
    let still = (0..frames.len()).filter(|&i| translate_in_asid(asid, at(i)).is_some()).count();
    window_back(false, frames.len() as u64, still)
}
