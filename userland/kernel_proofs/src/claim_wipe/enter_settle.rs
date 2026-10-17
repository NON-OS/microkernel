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

//! A release of A's claim settled in the enter model (L78), as
//! release::settle does it in one hold of the claims' lock: the last out of
//! the claim removes its entry, and gone::remove owes its wipe in that hold.

use super::enter_parts::{Entry, Knob};
use super::enter_stage::{gone, Stage};
use super::enter_world::World;

/// release::settle: one claims hold, a release uncounted; the last out
/// removes the entry, and gone::remove owes its wipe in that same hold
/// (OweApart: in the remover `p`'s next step instead).
pub(super) fn settle(w: &mut World, p: u8, k: Knob) -> bool {
    let named = |e: &Entry| matches!(e.stage, Stage::Leaving | Stage::Exiting);
    let Some(e) = w.entries[0].filter(named) else { return false };
    let e = Entry { leavers: e.leavers.saturating_sub(1), ..e };
    let last = gone(e.stage, u32::from(e.leavers), 0);
    w.entries[0] = if last { None } else { Some(e) };
    match (last, k) {
        (true, Knob::OweApart) => w.late = Some(p),
        (true, _) => w.owe(e.epoch),
        _ => {}
    }
    last
}

/// OweApart: the owe a settle left, as the remover's own next step.
pub(super) fn owe_late(w: &mut World, pc: u8) -> u8 {
    w.late = None;
    w.owe(1);
    pc
}
