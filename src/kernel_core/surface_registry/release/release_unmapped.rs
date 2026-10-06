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
use crate::kernel_core::surface_registry::table::{bump_generation, SLOTS};
use crate::kernel_core::surface_registry::types::{decode_handle, SurfaceHandle};

/*
 * Free the slot of a surface `owner` unmapped whole, once no other process
 * maps it (`pin/unmap_rule.rs`). Only a slot `handle` still names, still
 * `owner`'s and already without frames is freed, so a slot reused by a
 * later register is never touched. The owner's own attach record goes with
 * it, so a present through the old handle is refused instead of reading
 * pages that are no longer mapped.
 */
pub fn release_unmapped(owner: u32, handle: SurfaceHandle) {
    let (idx, epoch) = decode_handle(handle);
    {
        let mut slots = SLOTS.lock();
        let Some(entry) = slots.get_mut(idx as usize) else { return };
        let unmapped = entry
            .as_ref()
            .is_some_and(|s| s.epoch == epoch && s.owner_pid == owner && s.frames.is_empty());
        if !unmapped {
            return;
        }
        *entry = None;
        bump_generation(idx as usize);
    }
    attach_map::forget_handle(handle);
}
