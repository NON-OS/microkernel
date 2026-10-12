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
use crate::smp::lock_responsive;

use super::dma_held::HELD;
use super::dma_plan::hold_in;
use super::dma_rule::overlaps;
use super::gate::GATE;
use super::orphans::ORPHANS;

/*
 * `owner`'s DMA grant `[start, start + len)` is being released, its device
 * already cut off. True when a surface over its frames is attached by
 * another process: the registry keeps the frames, and `reclaim_dma` gives
 * them back once the last holder lets go. A surface nobody else holds stops
 * being attachable first, so no attach reaches the frames the caller then
 * frees. Under GATE, which every attach holds until its record is made.
 */
pub fn hold_dma(owner: u32, start: u64, len: u64) -> bool {
    let _gate = lock_responsive(&GATE);
    let live: Vec<(SurfaceHandle, Vec<PhysAddr>)> = {
        let mut slots = SLOTS.lock();
        let mut out = Vec::new();
        for (idx, entry) in slots.iter_mut().enumerate() {
            let Some(s) = entry.as_mut().filter(|s| s.owner_pid == owner) else { continue };
            if overlaps(s.frames.iter().map(|f| f.as_u64()), start, len) {
                let handle = encode_handle(idx as u32, s.epoch);
                out.push((handle, core::mem::take(&mut s.frames)));
            }
        }
        out
    };
    let foreign = has_foreign_holder;
    let page = |f: &PhysAddr| f.as_u64();
    let kept = hold_in((owner, start, len), live, &mut ORPHANS.lock(), foreign, page);
    let held = kept.is_some();
    HELD.lock().extend(kept);
    held
}
