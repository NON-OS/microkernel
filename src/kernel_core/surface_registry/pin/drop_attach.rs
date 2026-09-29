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

use super::orphans::reclaim;
use super::unmap_receiver::unmap_receiver;

/*
 * `pid` drops its attach of `handle`: its view of the frames, then its
 * attach record, then its reference. Orphaned frames no other process maps
 * any more are freed afterwards.
 */
pub fn drop_attach(pid: u32, handle: SurfaceHandle) -> Result<u32, RegistryError> {
    unmap_receiver(pid, handle);
    attach_map::forget(pid, handle);
    let result = release_surface(handle);
    reclaim();
    result
}
