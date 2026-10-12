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

use alloc::vec::Vec;
use core::sync::atomic::Ordering;

use crate::kernel_core::surface_registry::share::rule::Shares;
use crate::kernel_core::surface_registry::types::{
    encode_handle, RegistryError, SurfaceDescriptor, SurfaceHandle, SurfaceSid, FMT_ARGB8888,
    MAX_PAGES_PER_SURFACE, PIXEL_BYTES,
};
use crate::memory::addr::PhysAddr;

use super::{Slot, SLOTS, SLOT_GENERATIONS};

/// `grant`: the DMA run the frames belong to, when they are the owner's own
/// grant rather than its memory (L35).
pub fn register_surface(
    owner_pid: u32,
    desc: &SurfaceDescriptor,
    frames: Vec<PhysAddr>,
    grant: Option<(u64, u64)>,
) -> Result<(SurfaceSid, SurfaceHandle), RegistryError> {
    if desc.format != FMT_ARGB8888 || desc.width == 0 || desc.height == 0 {
        return Err(RegistryError::InvalidArg);
    }
    let min_stride = desc.width.checked_mul(PIXEL_BYTES).ok_or(RegistryError::InvalidArg)?;
    if desc.stride < min_stride {
        return Err(RegistryError::InvalidArg);
    }
    if frames.is_empty() || frames.len() > MAX_PAGES_PER_SURFACE {
        return Err(RegistryError::InvalidArg);
    }
    // Resolved before the table lock: it takes the process table's.
    let owner_group = crate::hardware::broker::holder_of(owner_pid);
    let mut slots = SLOTS.lock();
    for (i, entry) in slots.iter_mut().enumerate() {
        if entry.is_none() {
            let epoch = SLOT_GENERATIONS[i].load(Ordering::Acquire);
            *entry = Some(Slot {
                owner_pid,
                epoch,
                refcount: 1,
                width: desc.width,
                height: desc.height,
                stride: desc.stride,
                format: desc.format,
                flags: desc.flags,
                byte_len: desc.byte_len,
                owner_base_va: desc.base_va,
                frames,
                shares: Shares::new(owner_group),
                grant,
            });
            #[cfg(feature = "dbg-ring")]
            crate::log::dbg_ring::dbg_emit_2u64(0x5546_0002, i as u64, epoch as u64);
            return Ok((i as u64, encode_handle(i as u32, epoch)));
        }
    }
    Err(RegistryError::OutOfSlots)
}
