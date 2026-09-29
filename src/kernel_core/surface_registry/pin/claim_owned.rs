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

use super::frames_of::frames_of;
use super::window::Window;

/*
 * Surfaces `pid` registered from [addr, end) stop being attachable: their
 * frame list leaves the slot. A fully unmapped window that another process
 * still has attached becomes an orphan the registry frees later. A partly
 * unmapped attached window stays live and keeps the unmapped frames
 * allocated. Caller holds the gate, so no attach runs meanwhile.
 */
pub(super) fn claim_owned(pid: u32, addr: u64, end: u64, out: &mut Vec<Window>) {
    let mut hits: Vec<(usize, SurfaceHandle, bool)> = Vec::new();
    for (idx, entry) in SLOTS.lock().iter().enumerate() {
        let Some(slot) = entry.as_ref() else { continue };
        let base = slot.owner_base_va;
        let win_end = base.saturating_add(slot.frames.len() as u64 * 4096);
        if slot.owner_pid == pid && !slot.frames.is_empty() && base < end && addr < win_end {
            let whole = addr <= base && win_end <= end;
            hits.push((idx, encode_handle(idx as u32, slot.epoch), whole));
        }
    }
    for (idx, handle, whole) in hits {
        let attached = has_foreign_holder(handle, pid);
        let mut slots = SLOTS.lock();
        let Some(slot) = slots[idx].as_mut() else { continue };
        if encode_handle(idx as u32, slot.epoch) != handle || slot.owner_pid != pid {
            continue;
        }
        let base = slot.owner_base_va;
        if !attached {
            slot.frames = Vec::new();
        } else if whole {
            let mut w = Window::held(base, core::mem::take(&mut slot.frames));
            w.orphan = Some((pid, handle));
            out.push(w);
        } else {
            out.push(Window::held(base, slot.frames.clone()));
        }
    }
}

/* Windows `pid` attached from other owners that overlap [addr, end). */
pub(super) fn foreign_windows(pid: u32, addr: u64, end: u64, out: &mut Vec<Window>) {
    for (handle, base) in records_of(pid) {
        let Some((owner, frames)) = frames_of(handle) else { continue };
        let win_end = base.saturating_add(frames.len() as u64 * 4096);
        if owner != pid && base < end && addr < win_end {
            out.push(Window::held(base, frames));
        }
    }
}
