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

use crate::kernel_core::surface_registry::table::SLOTS;
use crate::kernel_core::surface_registry::types::{decode_handle, SurfaceHandle};
use crate::memory::addr::PhysAddr;

use super::orphans::ORPHANS;

/*
 * Owner and frame list of `handle`, by page index: the live slot's, or the
 * orphan's once the owner has unmapped the surface.
 */
pub(super) fn frames_of(handle: SurfaceHandle) -> Option<(u32, Vec<PhysAddr>)> {
    let (idx, epoch) = decode_handle(handle);
    {
        let slots = SLOTS.lock();
        if let Some(slot) = slots.get(idx as usize).and_then(|s| s.as_ref()) {
            if slot.epoch == epoch && !slot.frames.is_empty() {
                return Some((slot.owner_pid, slot.frames.clone()));
            }
        }
    }
    let orphans = ORPHANS.lock();
    orphans.iter().find(|o| o.handle == handle).map(|o| (o.owner, o.frames.clone()))
}
