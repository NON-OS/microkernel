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

//! Live surfaces over a DMA run that is going (L35).

use alloc::vec::Vec;

use crate::kernel_core::surface_registry::attach_map::snapshot::has_foreign_holder;
use crate::kernel_core::surface_registry::release::release_unmapped;
use crate::kernel_core::surface_registry::table::SLOTS;
use crate::kernel_core::surface_registry::types::{encode_handle, SurfaceHandle};

use super::orphans::{Orphan, ORPHANS};

/*
 * Every live surface over `run` loses its frames: the device and the owner
 * have lost them already. One another process maps becomes an orphan that
 * holds the run until that process lets go; any other goes now. The caller
 * holds the gate.
 */
pub(super) fn detach(run: (u64, u64)) {
    let live: Vec<(usize, SurfaceHandle, u32)> = SLOTS
        .lock()
        .iter()
        .enumerate()
        .filter_map(|(idx, e)| {
            let s = e.as_ref()?;
            (s.grant == Some(run) && !s.frames.is_empty())
                .then(|| (idx, encode_handle(idx as u32, s.epoch), s.owner_pid))
        })
        .collect();
    for (idx, handle, owner) in live {
        let attached = has_foreign_holder(handle, owner);
        let mut slots = SLOTS.lock();
        let Some(slot) = slots[idx].as_mut() else { continue };
        if encode_handle(idx as u32, slot.epoch) != handle {
            continue;
        }
        let frames = core::mem::take(&mut slot.frames);
        drop(slots);
        if attached {
            let (free, grant) = (Vec::new(), Some(run));
            ORPHANS.lock().push(Orphan { owner, handle, frames, free, grant });
        } else {
            release_unmapped(owner, handle);
        }
    }
}
