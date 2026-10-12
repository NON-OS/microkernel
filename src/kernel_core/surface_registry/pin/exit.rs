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

use crate::kernel_core::surface_registry::attach_map::snapshot::{has_foreign_holder, records_of};
use crate::kernel_core::surface_registry::table::SLOTS;
use crate::kernel_core::surface_registry::types::{encode_handle, SurfaceHandle};
use crate::smp::lock_responsive;

use super::frames_of::frames_of;
use super::gate::GATE;
use super::kept::keep;
use super::orphans::{Orphan, ORPHANS};

/*
 * `group`'s last live thread has ended; the tables `tables` names will be
 * freed frame by frame (L36). Under the gate, every surface it owns is
 * emptied, so a later attach fails. One another process still has attached
 * becomes an orphan `reclaim` frees after its last holder; frames it attached
 * from another owner are that owner's. Both are kept from the teardown. A
 * second pass, from finalize, finds nothing new.
 */
pub fn keep_at_exit(group: u32, tables: u64) {
    let _gate = lock_responsive(&GATE);
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
        let free = if grant.is_some() { Vec::new() } else { frames.clone() }; // the broker's
        ORPHANS.lock().push(Orphan { owner: group, handle, frames, free, grant });
    }
    for (handle, _) in records_of(group) {
        match frames_of(handle) {
            Some((owner, frames)) if owner != group => kept.extend_from_slice(&frames),
            _ => {}
        }
    }
    keep(tables, kept);
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
