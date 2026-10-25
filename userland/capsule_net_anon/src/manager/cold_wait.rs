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

//! The count of empty directory sweeps in a row, and what the last one met.
//! The capsule's manager runs on one thread; the atomics only keep this out
//! of the manager's state, which every other stage shares.

use core::sync::atomic::{AtomicBool, AtomicU32, Ordering};

use super::cold_wait_rule::{seconds, worth_saying};
use crate::trace;

static EMPTY: AtomicU32 = AtomicU32::new(0);
static NOT_UP: AtomicBool = AtomicBool::new(false);

/// A connect net.core refused because the network is not up yet.
pub fn not_up() {
    NOT_UP.store(true, Ordering::Relaxed);
}

/// When to sweep again, after one that left `held` certificates in hand.
pub fn after_sweep(held: usize, now: u64) -> u64 {
    let not_up = NOT_UP.swap(false, Ordering::Relaxed);
    if held > 0 {
        EMPTY.store(0, Ordering::Relaxed);
        return now.saturating_add(seconds(1));
    }
    let empty = EMPTY.load(Ordering::Relaxed).saturating_add(1);
    EMPTY.store(empty, Ordering::Relaxed);
    let wait = seconds(empty);
    if worth_saying(empty) {
        trace::say(if not_up {
            b"no authority reachable: the network is not up yet"
        } else {
            b"no authority answered"
        });
        trace::say_num(b"directory waits seconds", wait);
    }
    now.saturating_add(wait)
}
