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

//! How a switch updates the slots `on_cpu` describes.

use core::sync::atomic::Ordering;

use super::on_cpu::{this_cpu, LEAVING, OWNED, TRACKED};

/// What a switch replaced, so a switch that did not happen can be undone.
pub(crate) struct Undo {
    owned: u32,
    leaving: u32,
}

/// Record that this CPU is about to leave its current stack for `next`'s.
pub(crate) fn enter(next: u32) -> Undo {
    if !TRACKED {
        return Undo { owned: 0, leaving: 0 };
    }
    let me = this_cpu();
    let owned = OWNED[me].load(Ordering::Relaxed);
    let leaving = LEAVING[me].load(Ordering::Relaxed);
    if owned != next {
        if owned != 0 {
            LEAVING[me].store(owned, Ordering::SeqCst);
        }
        OWNED[me].store(next, Ordering::SeqCst);
    }
    Undo { owned, leaving }
}

/// The switch returned, so this CPU is still where it was before `enter`.
pub(crate) fn undo(prev: Undo) {
    if !TRACKED {
        return;
    }
    let me = this_cpu();
    OWNED[me].store(prev.owned, Ordering::SeqCst);
    LEAVING[me].store(prev.leaving, Ordering::SeqCst);
}

/// This CPU is off the stack it was leaving.
pub(crate) fn release_leaving() {
    if !TRACKED {
        return;
    }
    let slot = &LEAVING[this_cpu()];
    if slot.load(Ordering::Relaxed) != 0 {
        slot.store(0, Ordering::SeqCst);
    }
}

/// Adopt `pid` as this CPU's own when it runs one no switch recorded: the
/// first process, which the boot path starts by hand.
pub(crate) fn adopt_current(pid: u32) {
    if !TRACKED || pid == 0 {
        return;
    }
    let slot = &OWNED[this_cpu()];
    if slot.load(Ordering::Relaxed) == 0 {
        slot.store(pid, Ordering::SeqCst);
    }
}
