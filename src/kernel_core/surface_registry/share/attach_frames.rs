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

//! Giving a receiver its own view of a surface's frames.

use crate::kernel_core::surface_registry::attach_map::lookup;
use crate::kernel_core::surface_registry::release_surface;
use crate::kernel_core::surface_registry::table::SLOTS;
use crate::kernel_core::surface_registry::types::{
    decode_handle, RegistryError, SurfaceDescriptor, SurfaceHandle,
};

pub(super) fn attach_frames(
    receiver_pid: u32,
    handle: SurfaceHandle,
    out_desc: &mut SurfaceDescriptor,
    write: bool,
) -> Result<u64, RegistryError> {
    let (idx, epoch) = decode_handle(handle);
    let frames = {
        let mut slots = SLOTS.lock();
        let slot =
            slots.get_mut(idx as usize).and_then(|s| s.as_mut()).ok_or(RegistryError::BadHandle)?;
        if slot.epoch != epoch {
            #[cfg(feature = "dbg-ring")]
            crate::log::dbg_ring::dbg_emit_2u64(0x5546_0001, handle, slot.epoch as u64);
            return Err(RegistryError::BadHandle);
        }
        // Emptied at its owner's exit (`pin::exit`): nothing left to map.
        if slot.frames.is_empty() {
            return Err(RegistryError::BadHandle);
        }
        slot.refcount = slot.refcount.checked_add(1).ok_or(RegistryError::InvalidArg)?;
        slot.frames.clone()
    };
    let mapped = super::map_frames::map_view(receiver_pid, handle, &frames, out_desc, write);
    // A view left mapped is recorded as an attach and keeps its reference.
    // At none left, the slot goes as `release_surface` frees it (L49).
    if mapped.is_err() && lookup(receiver_pid, handle).is_none() {
        let _ = release_surface(handle);
    }
    mapped
}
