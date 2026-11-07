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


//! A voluntary yield's next process, named by the caller. A guest that
//! traps hands the CPU to its supervisor, and the supervisor that answers
//! hands it back: picked round robin instead, each hop of a Linux call
//! waited behind every other runnable process, often for a tick.

use core::sync::atomic::{AtomicU64, Ordering};

const SLOTS: usize = 64;
static HINTS: [AtomicU64; SLOTS] = [const { AtomicU64::new(0) }; SLOTS];

fn slot(pid: u32) -> &'static AtomicU64 {
    &HINTS[pid as usize % SLOTS]
}

/// `from`'s next yield goes to `to`, when it may run then (band_choice::prefer).
pub(crate) fn hint(from: u32, to: u32) {
    if from != 0 && to != 0 && from != to {
        slot(from).store((u64::from(from) << 32) | u64::from(to), Ordering::Relaxed);
    }
}

/// The process `from` named, taken once.
pub(super) fn take(from: u32) -> Option<u32> {
    let s = slot(from);
    let v = s.load(Ordering::Relaxed);
    if v >> 32 != u64::from(from) {
        return None;
    }
    s.compare_exchange(v, 0, Ordering::AcqRel, Ordering::Relaxed).ok().map(|v| v as u32)
}
