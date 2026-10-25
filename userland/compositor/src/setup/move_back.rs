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

//! Going back to the display the desktop moved off, once its device is back
//! (display/return_rule.rs). What was left is one word here rather than a
//! field of the context: it outlives every primary adopted meanwhile, and
//! only the poll and the move off read or write it.

use core::sync::atomic::{AtomicU64, Ordering};

use crate::display::return_rule::{back, remember, worth, Left};
use crate::state::Context;

use super::discover::{displays::DISPLAYS, lookup_at};

static LEFT: AtomicU64 = AtomicU64::new(0);

fn rank(name: &str) -> Option<usize> {
    DISPLAYS.iter().position(|n| *n == name)
}

fn keep(kept: Option<Left>) {
    LEFT.store(Left::word(kept), Ordering::Relaxed);
}

/// The display named `name`, whose driver `pid` said its device left, was
/// moved off.
pub fn note_left(name: &str, pid: u32) {
    let Some(rank) = rank(name) else { return };
    let kept = Left::from_word(LEFT.load(Ordering::Relaxed));
    keep(remember(kept, Left { rank, pid }));
}

/// From the poll of a steady driver: back to the display left, primed and
/// adopted, once its driver is announced again. One that does not prime
/// yet is tried again at the next poll.
pub fn move_back(ctx: &mut Context) -> Option<&'static str> {
    let driving = rank(ctx.wire?.name)?;
    let kept = worth(Left::from_word(LEFT.load(Ordering::Relaxed)), driving);
    keep(kept);
    let ep = lookup_at(kept?.rank);
    if !back(kept, driving, ep.map(|e| e.pid)) {
        return None;
    }
    let next = super::prime_wire::prime(ep?).ok()?;
    super::adopt::adopt(ctx, next);
    keep(None);
    Some("its device came back; moved back to it")
}
