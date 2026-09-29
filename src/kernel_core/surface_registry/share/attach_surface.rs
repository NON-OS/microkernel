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
    if let Some((base_va, byte_len)) = super::super::attach_map::lookup(receiver_pid, handle) {
        *out_desc = super::descriptor::descriptor(handle)?;
        out_desc.base_va = base_va;
        out_desc.byte_len = byte_len;
        return Ok(base_va);
    }
    if let Some(base_va) = self_attach(receiver_pid, handle, out_desc)? {
        return Ok(base_va);
    }
    attach_frames(receiver_pid, handle, out_desc)
}
