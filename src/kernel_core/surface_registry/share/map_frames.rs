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

//! A receiver's view of a surface's frames, whole or not at all.

use crate::kernel_core::surface_registry::attach_map::record;
use crate::kernel_core::surface_registry::types::{
    RegistryError, SurfaceDescriptor, SurfaceHandle,
};
use crate::memory::addr::{PhysAddr, VirtAddr};
use crate::memory::paging::manager::api::{map_page_in_asid, unmap_page_in_asid};
use crate::memory::paging::types::PagePermissions;
use crate::process::current_process;

pub(super) fn map_view(
    receiver_pid: u32,
    handle: SurfaceHandle,
    frames: &[PhysAddr],
    out_desc: &mut SurfaceDescriptor,
) -> Result<u64, RegistryError> {
    let mut desc = super::descriptor::descriptor(handle)?;
    let asid = super::super::space::space_of(receiver_pid).ok_or(RegistryError::MapFailed)?;
    let proc = current_process().ok_or(RegistryError::NoProc)?;
    let len = frames.len().saturating_mul(4096);
    let base = proc.reserve_vma(len, asid).map_err(|_| RegistryError::MapFailed)?;
    // An munmap there read its held frames before this view existed (L50).
    if super::super::pin::busy::busy_over(base.as_u64(), len as u64) {
        proc.unreserve_vma(base, len);
        return Err(RegistryError::MapFailed);
    }
    if let Err(stuck) = map_frames(asid, base, frames) {
        // A page that would not come down still maps the frames: recorded,
        // it holds them until the receiver detaches or exits.
        if stuck {
            record(receiver_pid, handle, base.as_u64(), desc.byte_len);
        } else {
            proc.unreserve_vma(base, len);
        }
        return Err(RegistryError::MapFailed);
    }
    desc.base_va = base.as_u64();
    *out_desc = desc;
    record(receiver_pid, handle, base.as_u64(), out_desc.byte_len);
    Ok(base.as_u64())
}

/*
 * Map `frames` user read-write at consecutive pages from `base` in `asid`.
 * When one fails, the pages already mapped come down again: left with no
 * attach record, they outlived the frames (L49). Err(true) if one would not.
 */
fn map_frames(asid: u32, base: VirtAddr, frames: &[PhysAddr]) -> Result<(), bool> {
    let perms = PagePermissions::user_rw();
    let page = |i: usize| VirtAddr::new(base.as_u64() + (i as u64) * 4096);
    for (i, frame) in frames.iter().enumerate() {
        if map_page_in_asid(asid, page(i), *frame, perms).is_err() {
            let stuck = (0..i).filter(|&j| unmap_page_in_asid(asid, page(j), perms).is_err());
            return Err(stuck.count() > 0);
        }
    }
    Ok(())
}
