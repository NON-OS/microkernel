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

//! A step's act put back on its record (L78). An ended wipe first settles
//! the function's kept frames, then leaves the record its end decides: none
//! after a clean reset, kept for its holder's service, or lost for its hold;
//! the end is said once, outside every lock.

use core::sync::atomic::Ordering;

use super::end_rule::{ended, frees, End};
use super::table::{Wipe, PENDING, WIPES};
use crate::drivers::pci::ConfigSpace;

pub(super) fn put(w: Wipe, end: Option<End>, now: u64) {
    if end == Some(End::Unreset) {
        super::check::unmark(&ConfigSpace::new(w.address), &w);
    }
    // Settled while the record still reads as a wipe under way, so a claim
    // that reads its end also reads its frames kept for good (L80). A lost
    // function may still come back: its frames wait for its next end.
    let (freed, kept) = match end {
        None | Some(End::Lost) => (0, 0),
        Some(e) => crate::hardware::broker::dma::settle_kept(
            w.device_id,
            w.epoch,
            frees(e, w.bme_off, w.caps.binds),
        ),
    };
    {
        let mut all = crate::smp::lock_responsive(&WIPES);
        let key = |r: &Wipe| (r.device_id, r.epoch) == (w.device_id, w.epoch);
        let Some(at) = all.iter().position(key) else { return };
        match end.map(|e| ended(e, w.caps.display, now)) {
            None => all[at] = Wipe { busy: false, ..w },
            Some(None) => drop(all.remove(at)),
            Some(Some(phase)) => all[at] = Wipe { phase, busy: false, ..w },
        }
        if end.is_some() {
            PENDING.fetch_sub(1, Ordering::AcqRel);
        }
    }
    let Some(end) = end else { return };
    say(&w, end, freed, kept);
}

fn say(w: &Wipe, end: End, freed: usize, kept: usize) {
    let how = match end {
        End::Clean if !w.caps.display => "reset",
        End::Clean => "reset; kept for its own driver service, as a display function",
        End::Unreset => "not reset: kept for its own driver service",
        End::Lost => "not back within its window: refused for a while, then asked again",
    };
    crate::log::info!(
        "[BROKER] {} {:?} {}; DMA runs freed {} kept {}",
        w.address,
        w.method,
        how,
        freed,
        kept
    );
}
