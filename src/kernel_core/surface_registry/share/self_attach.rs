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

//! The owner attaching its own surface.

use crate::kernel_core::surface_registry::table::SLOTS;
use crate::kernel_core::surface_registry::types::{
    decode_handle, RegistryError, SurfaceDescriptor, SurfaceHandle,
};

/// The owner's own va when the owner is the receiver, or `None` when the
/// receiver is somebody else and a real mapping has to be made.
pub(super) fn self_attach(
    receiver_pid: u32,
    handle: SurfaceHandle,
    out_desc: &mut SurfaceDescriptor,
) -> Result<Option<u64>, RegistryError> {
    let (idx, epoch) = decode_handle(handle);
    let slots = SLOTS.lock();
    let slot = slots.get(idx as usize).and_then(|s| s.as_ref()).ok_or(RegistryError::BadHandle)?;
    if slot.epoch != epoch {
        #[cfg(feature = "dbg-ring")]
        crate::log::dbg_ring::dbg_emit_2u64(0x5546_0001, handle, slot.epoch as u64);
        return Err(RegistryError::BadHandle);
    }
    if slot.owner_pid != receiver_pid || slot.owner_base_va == 0 {
        return Ok(None);
    }
    let (base_va, byte_len) = (slot.owner_base_va, slot.byte_len);
    drop(slots);
    *out_desc = super::descriptor::descriptor(handle)?;
    out_desc.base_va = base_va;
    out_desc.byte_len = byte_len;
    super::super::attach_map::record(receiver_pid, handle, base_va, byte_len);
    Ok(Some(base_va))
}
