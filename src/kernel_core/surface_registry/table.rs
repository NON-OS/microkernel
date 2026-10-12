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
use core::sync::atomic::{AtomicU32, Ordering};
use spin::Mutex;

use crate::memory::addr::PhysAddr;

use super::share::rule::Shares;
use super::types::{encode_handle, RegistryError, SurfaceHandle, SurfaceSid, SLOT_CAP};

mod register;

pub use register::register_surface;

pub(super) struct Slot {
    pub(super) owner_pid: u32,
    pub(super) epoch: u32,
    pub(super) refcount: u32,
    pub(super) width: u32,
    pub(super) height: u32,
    pub(super) stride: u32,
    pub(super) format: u32,
    pub(super) flags: u64,
    pub(super) byte_len: u64,
    // The VA the owner registered the surface at. The owner already has
    // it mapped, so a self-attach returns this directly instead of
    // remapping the frames at a fresh VA.
    pub(super) owner_base_va: u64,
    pub(super) frames: Vec<PhysAddr>,
    /// Who may attach besides the owner (L18c).
    pub(super) shares: Shares,
}

pub(super) static SLOTS: Mutex<[Option<Slot>; SLOT_CAP]> = Mutex::new([const { None }; SLOT_CAP]);

pub(super) static SLOT_GENERATIONS: [AtomicU32; SLOT_CAP] = [const { AtomicU32::new(1) }; SLOT_CAP];

pub(super) fn bump_generation(idx: usize) {
    let next = SLOT_GENERATIONS[idx].fetch_add(1, Ordering::AcqRel).wrapping_add(1);
    if next == 0 {
        SLOT_GENERATIONS[idx].store(1, Ordering::Release);
    }
}

pub fn lookup_owned(owner_pid: u32, sid: SurfaceSid) -> Result<SurfaceHandle, RegistryError> {
    let slots = SLOTS.lock();
    let idx = sid as usize;
    let slot = slots.get(idx).and_then(|s| s.as_ref()).ok_or(RegistryError::NotFound)?;
    if slot.owner_pid != owner_pid {
        return Err(RegistryError::NotOwner);
    }
    Ok(encode_handle(idx as u32, slot.epoch))
}
