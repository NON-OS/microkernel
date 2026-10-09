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

use crate::kernel_core::surface_registry::attach_map::state::ATTACHES;
use crate::kernel_core::surface_registry::types::SurfaceHandle;

/* True when a process other than `owner` holds an attach record for `handle`. */
pub fn has_foreign_holder(handle: SurfaceHandle, owner: u32) -> bool {
    ATTACHES.lock().iter().any(|r| r.handle == handle && r.pid != owner)
}

/* (handle, base_va) of every attach record held by `pid`. */
pub fn records_of(pid: u32) -> Vec<(SurfaceHandle, u64)> {
    ATTACHES.lock().iter().filter(|r| r.pid == pid).map(|r| (r.handle, r.base_va)).collect()
}
