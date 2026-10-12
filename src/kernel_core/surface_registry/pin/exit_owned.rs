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

use crate::kernel_core::surface_registry::attach_map::snapshot::has_foreign_holder;
use crate::kernel_core::surface_registry::table::SLOTS;
use crate::kernel_core::surface_registry::types::{encode_handle, SurfaceHandle};
use crate::memory::addr::PhysAddr;

use super::dma_rule::strip;
use super::orphans::{Orphan, ORPHANS};

/*
 * For keep_at_exit, under GATE: each of `pid`'s surfaces another process
 * still attaches becomes an orphan, and its frames are returned for the
 * address space's teardown to leave alone.
 */
pub(super) fn orphan_owned(pid: u32) -> Vec<PhysAddr> {
    let dma = crate::hardware::broker::dma::ranges_of(pid);
    let mut keep = Vec::new();
    let owned: Vec<(usize, SurfaceHandle)> = SLOTS
        .lock()
        .iter()
        .enumerate()
        .filter_map(|(idx, e)| {
            let s = e.as_ref()?;
            (s.owner_pid == pid && !s.frames.is_empty())
                .then(|| (idx, encode_handle(idx as u32, s.epoch)))
        })
        .collect();
    for (idx, handle) in owned {
        if !has_foreign_holder(handle, pid) {
            continue;
        }
        let mut slots = SLOTS.lock();
        let Some(slot) = slots[idx].as_mut() else { continue };
        if encode_handle(idx as u32, slot.epoch) != handle || slot.owner_pid != pid {
            continue;
        }
        let frames = core::mem::take(&mut slot.frames);
        drop(slots);
        keep.extend_from_slice(&frames);
        // A DMA grant's frames are the broker's, given back through it (K15).
        let mut free = frames.clone();
        for &(start, len) in &dma {
            strip(&mut free, |f| f.as_u64(), start, len);
        }
        ORPHANS.lock().push(Orphan { owner: pid, handle, frames, free });
    }
    keep
}
