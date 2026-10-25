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
use crate::memory::addr::PhysAddr;
use crate::memory::paging::types::PagePermissions;
use crate::process::current_process;

/// `write`: the receiver was named to write; else its view is read-only (K20).
pub(super) fn map_view(
    receiver_pid: u32,
    handle: SurfaceHandle,
    frames: &[PhysAddr],
    out_desc: &mut SurfaceDescriptor,
    write: bool,
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
    let perms = if write { PagePermissions::user_rw() } else { PagePermissions::user_ro() };
    if let Err(stuck) = super::map_pages::map_pages(asid, base, frames, perms) {
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
