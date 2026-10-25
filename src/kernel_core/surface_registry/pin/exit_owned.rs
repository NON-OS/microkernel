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
use super::process_of::same_process;

/*
 * For keep_at_exit, under the gate: every surface `group` owns loses its
 * frames. One another process still attaches becomes an orphan held by the
 * dying tables until they are freed (L49), and its frames are returned for
 * that teardown to leave alone. A DMA run's frames are the broker's, given
 * back through it, never page by page (L35, K15): none of a surface over its
 * owner's own grant, and none of any grant a thread of the group holds.
 */
pub(super) fn orphan_owned(group: u32, tables: u64) -> Vec<PhysAddr> {
    let dma: Vec<(u32, u64, u64)> = crate::hardware::broker::dma::grant_ranges()
        .into_iter()
        .filter(|&(holder, _, _)| same_process(holder, group))
        .collect();
    let mut kept = Vec::new();
    for (idx, handle) in owned_by(group) {
        let mut slots = SLOTS.lock();
        let Some(slot) = slots[idx].as_mut() else { continue };
        if encode_handle(idx as u32, slot.epoch) != handle || slot.owner_pid != group {
            continue;
        }
        let frames = core::mem::take(&mut slot.frames);
        let grant = slot.grant;
        drop(slots);
        if frames.is_empty() || !has_foreign_holder(handle, group) {
            continue;
        }
        kept.extend_from_slice(&frames);
        let mut free = if grant.is_some() { Vec::new() } else { frames.clone() };
        for &(_, start, len) in &dma {
            strip(&mut free, |f| f.as_u64(), start, len);
        }
        ORPHANS.lock().push(Orphan { owner: group, handle, frames, free, grant, tables });
    }
    kept
}

fn owned_by(group: u32) -> Vec<(usize, SurfaceHandle)> {
    SLOTS
        .lock()
        .iter()
        .enumerate()
        .filter_map(|(idx, e)| {
            let s = e.as_ref()?;
            (s.owner_pid == group).then(|| (idx, encode_handle(idx as u32, s.epoch)))
        })
        .collect()
}
