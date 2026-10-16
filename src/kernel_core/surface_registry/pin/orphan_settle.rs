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

//! An owner's munmap of a surface another process still maps (L54). The
//! orphan is listed the moment the slot gives up its frames, still under the
//! gate, and the frames the munmap really took down are added once it has
//! settled. Listed only at the end, a holder's release in between found no
//! frames, left its view mapped, and `reclaim` then freed the frames under it.

use alloc::vec::Vec;

use crate::kernel_core::surface_registry::types::SurfaceHandle;
use crate::memory::addr::PhysAddr;

use super::gate::GATE;
use super::orphans::{Orphan, ORPHANS};
use super::own_orphans::OwnOrphans;
use super::window::Window;

/// List `handle`'s frames as an orphan, nothing yet to free.
pub(super) fn publish(owner: u32, handle: SurfaceHandle, frames: Vec<PhysAddr>) {
    let free = Vec::new();
    ORPHANS.lock().push(Orphan { owner, handle, frames, free, grant: None, tables: 0 });
}

/// Hand over what each window's munmap unmapped, and what it popped of a
/// sibling's orphan, under the gate; then free what nobody maps.
pub(super) fn settle(windows: Vec<Window>, own: OwnOrphans) {
    {
        let _gate = crate::smp::lock_responsive(&GATE);
        for w in windows {
            if let Some((owner, handle)) = w.orphan {
                settle_one(owner, handle, w.frames, w.unmapped);
            }
        }
        own.settle();
    }
    super::orphans::reclaim();
}

/// When the orphan was already freed (its last holder let go meanwhile), it
/// is listed again to free just what was unmapped. A frame already listed to
/// free is not listed twice, or `reclaim` would free it twice.
pub(super) fn settle_one(
    owner: u32,
    handle: SurfaceHandle,
    frames: Vec<PhysAddr>,
    unmapped: Vec<PhysAddr>,
) {
    let mut list = ORPHANS.lock();
    match list.iter_mut().find(|o| o.handle == handle && o.owner == owner) {
        Some(o) => {
            o.free.extend(unmapped);
            o.free.sort_unstable_by_key(|f| f.as_u64());
            o.free.dedup();
        }
        None => list.push(Orphan { owner, handle, frames, free: unmapped, grant: None, tables: 0 }),
    }
}
