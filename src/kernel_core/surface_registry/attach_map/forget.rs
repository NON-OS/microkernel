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

use crate::kernel_core::surface_registry::attach_map::state::ATTACHES;
use crate::kernel_core::surface_registry::types::SurfaceHandle;

/// Whether a record was there to forget: two releases of one record race to
/// here, and only the one that removed it gives back its reference (L54).
pub fn forget(pid: u32, handle: SurfaceHandle) -> bool {
    let mut v = ATTACHES.lock();
    let before = v.len();
    v.retain(|r| !(r.pid == pid && r.handle == handle));
    v.len() != before
}

/// Forget the one record `pid` holds for `handle` at `base`: a record an
/// attach wrote after the view at `base` came down is another view, and it
/// keeps its own record and its own reference (L57).
pub fn forget_at(pid: u32, handle: SurfaceHandle, base: u64) -> bool {
    let mut v = ATTACHES.lock();
    let at = v.iter().position(|r| r.pid == pid && r.handle == handle && r.base_va == base);
    at.map(|i| v.swap_remove(i)).is_some()
}
