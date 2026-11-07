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

use crate::kernel_core::surface_registry::table::SLOTS;
use crate::kernel_core::surface_registry::types::{
    decode_handle, RegistryError, SurfaceDescriptor, SurfaceHandle,
};

/// `handle`'s descriptor as an attach hands it out. Its owner_pid is the
/// process that registered the surface, answerable for it (answerable.rs):
/// its owner, or for a surface pinned from a guest, the guest's supervisor,
/// so a compositor can tell whether the process naming it may show it.
pub(super) fn descriptor(handle: SurfaceHandle) -> Result<SurfaceDescriptor, RegistryError> {
    let (idx, epoch) = decode_handle(handle);
    let mut desc = {
        let slots = SLOTS.lock();
        let slot =
            slots.get(idx as usize).and_then(|s| s.as_ref()).ok_or(RegistryError::BadHandle)?;
        if slot.epoch != epoch {
            return Err(RegistryError::BadHandle);
        }
        SurfaceDescriptor {
            width: slot.width,
            height: slot.height,
            stride: slot.stride,
            format: slot.format,
            byte_len: slot.byte_len,
            base_va: 0,
            flags: slot.flags,
            owner_pid: slot.owner_pid,
            writable: 0,
        }
    };
    // After the table's lock: the foreign registry's is never taken under it.
    let supervisor = crate::process::foreign::supervisor_of(desc.owner_pid);
    let supervisor = supervisor.map(crate::hardware::broker::holder_of);
    desc.owner_pid = super::answerable::answerable(desc.owner_pid, supervisor);
    Ok(desc)
}
