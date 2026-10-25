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

use crate::kernel_core::surface_registry::attach_map;
use crate::kernel_core::surface_registry::table::{register_surface, SLOTS};
use crate::kernel_core::surface_registry::types::{
    RegistryError, SurfaceDescriptor, SurfaceHandle, SurfaceSid,
};
use crate::memory::addr::{PhysAddr, VirtAddr};
use crate::memory::paging::manager::api::translate_address;
use crate::smp::lock_responsive;

use super::busy::busy_over;
use super::gate::GATE;
use super::orphans::ORPHANS;
use super::register_race::still_mapped;
use super::register_rule::may_register;

/*
 * Register `frames` for `pid` only if they are its own to give
 * (`register_rule`) and still where the walk found them (`register_race`).
 * The checks, the slot and the owner's attach record are made under the pin
 * gate: an exit, an attach or a second registration between the checks and
 * the slot would make them stale.
 */
pub fn register_owned(
    pid: u32,
    desc: &SurfaceDescriptor,
    frames: Vec<PhysAddr>,
) -> Result<(SurfaceSid, SurfaceHandle), RegistryError> {
    let _gate = lock_responsive(&GATE);
    let mut new: Vec<u64> = frames.iter().map(|f| f.as_u64()).collect();
    let now = (0..new.len() as u64).map(|i| {
        let va = VirtAddr::new(desc.base_va + i * 4096);
        translate_address(va).map(|pa| pa.as_u64() & !0xFFF)
    });
    if busy_over(desc.base_va, desc.byte_len) || !still_mapped(&new, now) {
        return Err(RegistryError::InvalidArg);
    }
    let device = crate::hardware::broker::touches_device_window(desc.base_va, desc.byte_len);
    let grant = super::register_grant::own_grant(pid, desc, device);
    new.sort_unstable();
    {
        // Nowhere else holds both; taken in this order, nothing can wait on
        // the other way round.
        let slots = SLOTS.lock();
        let orphans = ORPHANS.lock();
        let live = slots.iter().flatten().flat_map(|s| s.frames.iter());
        let orphaned = orphans.iter().flat_map(|o| o.frames.iter());
        may_register(device && grant.is_none(), &new, live.chain(orphaned).map(|f| f.as_u64()))
            .map_err(|_| RegistryError::NotOwner)?;
    }
    let (sid, handle) = register_surface(pid, desc, frames, grant)?;
    attach_map::record(pid, handle, desc.base_va, desc.byte_len);
    Ok((sid, handle))
}
