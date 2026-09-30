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
use super::on_cpu_space::SPACE_LEFT;

/// What a switch replaced, so a switch that did not happen can be undone.
pub(crate) struct Undo {
    owned: u32,
    leaving: u32,
    space: u32,
}

/// Record that this CPU is about to leave its current stack for `next`'s.
pub(crate) fn enter(next: u32) -> Undo {
    if !TRACKED {
        return Undo { owned: 0, leaving: 0, space: 0 };
    }
    let me = this_cpu();
    let owned = OWNED[me].load(Ordering::Relaxed);
    let leaving = LEAVING[me].load(Ordering::Relaxed);
    let space = SPACE_LEFT[me].load(Ordering::Relaxed);
    if owned != next {
        if owned != 0 {
            LEAVING[me].store(owned, Ordering::SeqCst);
        }
        OWNED[me].store(next, Ordering::SeqCst);
    }
    /*
     * Cleared even when `next` is the pid owned: a recycled pid's tables are
     * about to be loaded, and the record would hide them.
     */
    if space != 0 {
        SPACE_LEFT[me].store(0, Ordering::SeqCst);
    }
    Undo { owned, leaving, space }
}

/// The switch returned, so this CPU is still where it was before `enter`.
pub(crate) fn undo(prev: Undo) {
    if !TRACKED {
        return;
    }
    let me = this_cpu();
    OWNED[me].store(prev.owned, Ordering::SeqCst);
    LEAVING[me].store(prev.leaving, Ordering::SeqCst);
    SPACE_LEFT[me].store(prev.space, Ordering::SeqCst);
}
