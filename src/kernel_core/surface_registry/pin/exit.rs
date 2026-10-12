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

use alloc::collections::BTreeMap;
use alloc::vec::Vec;

use spin::Mutex;

use crate::kernel_core::surface_registry::attach_map::snapshot::records_of;
use crate::memory::addr::PhysAddr;
use crate::smp::lock_responsive;

use super::frames_of::frames_of;
use super::gate::GATE;

/*
 * The frames each exiting process's address space must not free, from its
 * exit until its tables are torn down.
 */
static KEPT: Mutex<BTreeMap<u32, Vec<PhysAddr>>> = Mutex::new(BTreeMap::new());

/*
 * `pid` is exiting, and its address space will be freed frame by frame. Two
 * kinds of frame in it are not its to free. Frames of its own surfaces that
 * another process (the compositor) still has attached become orphans, freed
 * by `reclaim` once the last holder releases its attach: freed with the
 * process, they went on being composited after the next allocation took
 * them, and a window showed pieces of whatever was decoded there. Frames of
 * surfaces it attached from another owner are the owner's to free; freed
 * here too, they were freed twice. Runs before the exit drops the slots.
 */
pub fn keep_at_exit(pid: u32) {
    let _gate = lock_responsive(&GATE);
    let mut keep = super::exit_owned::orphan_owned(pid);
    for (handle, _) in records_of(pid) {
        if let Some((owner, frames)) = frames_of(handle) {
            if owner != pid {
                keep.extend_from_slice(&frames);
            }
        }
    }
    keep.sort_unstable_by_key(|f| f.as_u64());
    keep.dedup();
    if !keep.is_empty() {
        KEPT.lock().insert(pid, keep);
    }
}

/* The frames `pid`'s teardown leaves alone, handed over once. */
pub fn take_kept(pid: u32) -> Vec<PhysAddr> {
    KEPT.lock().remove(&pid).unwrap_or_default()
}
