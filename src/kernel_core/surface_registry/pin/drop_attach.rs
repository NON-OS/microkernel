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

use crate::kernel_core::surface_registry::attach_map;
use crate::kernel_core::surface_registry::release_surface;
use crate::kernel_core::surface_registry::types::{RegistryError, SurfaceHandle};
use crate::smp::lock_responsive;

use super::gate::GATE;
use super::orphans::reclaim;
use super::unmap_receiver::unmap_receiver;

/*
 * `pid` drops its attach of `handle`: its view of the frames, then its
 * attach record, then its reference. Orphaned frames no other process maps
 * any more are freed afterwards. One drop at a time finds the record, takes
 * its view down and forgets that record, so a second release racing this one
 * never forgets the record of a view an attach mapped in between (L57).
 */
pub fn drop_attach(pid: u32, handle: SurfaceHandle) -> Result<u32, RegistryError> {
    {
        let _gate = lock_responsive(&GATE);
        let (base, _) = attach_map::lookup(pid, handle).ok_or(RegistryError::NotFound)?;
        if !unmap_receiver(pid, handle, base) {
            return Err(RegistryError::MapFailed);
        }
        if !attach_map::forget_at(pid, handle, base) {
            return Err(RegistryError::NotFound);
        }
    }
    /*
     * A surface whose owner has exited has no slot left; its frames are an
     * orphan, and this holder letting go is the release. Refused, the holder
     * kept a view whose pages had just been unmapped.
     */
    let result = match release_surface(handle) {
        Err(RegistryError::BadHandle) => Ok(0),
        other => other,
    };
    reclaim();
    result
}
