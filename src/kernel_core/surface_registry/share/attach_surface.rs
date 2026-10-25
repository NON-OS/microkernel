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

//! Handing a surface to another process.

use crate::kernel_core::surface_registry::types::{
    RegistryError, SurfaceDescriptor, SurfaceHandle,
};

use super::attach_frames::attach_frames;
use super::rights_attach::Attach;
use super::self_attach::self_attach;

pub fn attach_surface(
    receiver_pid: u32,
    handle: SurfaceHandle,
    out_desc: &mut SurfaceDescriptor,
) -> Result<u64, RegistryError> {
    // Never to a guest.
    if crate::process::foreign::is_foreign(receiver_pid) {
        return Err(RegistryError::InvalidArg);
    }
    // Before any record is read: a handle is easy to guess (L18c).
    super::permit::permit(receiver_pid, handle)?;
    /* Held until the mapping is recorded; see `pin::gate`. */
    let _gate = crate::smp::lock_responsive(&super::super::pin::gate::GATE);
    // The owner, or a receiver it named; anyone else learns nothing.
    let write = match super::rights::may_attach(handle, receiver_pid) {
        Attach::Owner => true,
        Attach::Receiver { write } => write,
        Attach::Refused => return Err(RegistryError::NotOwner),
    };
    // Its owner is exiting: keep_at_exit has already counted the holders.
    if !super::super::pin::closing::attachable(handle) {
        return Err(RegistryError::BadHandle);
    }
    let base_va = view(receiver_pid, handle, out_desc, write)?;
    out_desc.writable = u32::from(write);
    Ok(base_va)
}

fn view(
    pid: u32,
    handle: SurfaceHandle,
    out_desc: &mut SurfaceDescriptor,
    write: bool,
) -> Result<u64, RegistryError> {
    if let Some(done) = super::existing::existing(pid, handle, out_desc) {
        return done;
    }
    if let Some(base_va) = self_attach(pid, handle, out_desc)? {
        return Ok(base_va);
    }
    attach_frames(pid, handle, out_desc, write)
}
