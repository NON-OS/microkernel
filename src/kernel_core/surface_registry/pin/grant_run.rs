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

//! A DMA grant's run that a surface still holds (L35). A GPU driver's
//! scanout is its own grant, mapped by the compositor. When the grant goes,
//! at an unmap, a release or the driver's exit, the device loses it at once;
//! its frames wait here while another process maps them, and go back to the
//! broker when the last one lets go.

use alloc::vec::Vec;
use spin::Mutex;

use crate::smp::lock_responsive;

use super::gate::GATE;
use super::orphans::ORPHANS;

/* Runs the broker left with the registry, as (physical start, length). */
static RUNS: Mutex<Vec<(u64, u64)>> = Mutex::new(Vec::new());

/*
 * The broker is tearing down the grant whose run is (`start`, `length`).
 * True when the registry keeps the run, because a surface over it is still
 * mapped by another process; the broker frees it otherwise.
 */
pub fn hold(start: u64, length: u64) -> bool {
    let run = (start, length);
    let _gate = lock_responsive(&GATE);
    super::grant_detach::detach(run);
    let held = ORPHANS.lock().iter().any(|o| o.grant == Some(run));
    if held {
        RUNS.lock().push(run);
    }
    held
}

/*
 * Give back every run no orphan holds any more: chosen under GATE, which
 * `reclaim` has let go before it calls this, and freed once it is let go
 * again, so the broker's free never runs inside it (reclaim_rule.rs).
 */
pub(super) fn free_unheld() {
    let mut free = Vec::new();
    {
        let _gate = lock_responsive(&GATE);
        let held: Vec<(u64, u64)> = ORPHANS.lock().iter().filter_map(|o| o.grant).collect();
        RUNS.lock().retain(|run| {
            let keep = held.contains(run);
            if !keep {
                free.push(*run);
            }
            keep
        });
    }
    for (start, length) in free {
        crate::hardware::broker::dma_free_held_run(start, length);
    }
}
