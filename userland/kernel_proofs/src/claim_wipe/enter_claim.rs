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

//! A claimant's call in the enter model (L78), in claim.rs's order: enter's
//! step_all, its one claims hold that asks the record and pushes Making, the
//! steps between, then admit or withdraw (enter_admit).

use super::admit_rule::Admit;
use super::enter_again::{again, push, rearm, taken};
use super::enter_events::{BY_CLAIM, BY_WAIT, REFUSED};
use super::enter_parts::{Knob, OUT};
use super::enter_stage::{over, ClaimError};
use super::enter_wipe::pass;
use super::enter_world::World;

/// Claimant `c`'s next step (B is 1, A2 is 2); its path is `c + 2`.
pub(super) fn claimant(w: &mut World, c: u8, k: Knob) -> bool {
    let p = usize::from(c) + 2;
    let pc = w.pc[p];
    match pc {
        // hold::admits, wipe::fresh and dma::kept_bytes change nothing held
        // here; then enter's step_all(true), one pass of the wipe.
        0..=2 => w.pc[p] = if pass(w, pc, k, BY_CLAIM) { pc + 1 } else { 3 },
        3 => enter(w, c, k),
        4 => push(w, c, k),
        // claim.rs between enter and admit takes neither lock, but a step
        // may refuse the claim (attach, restore), which then withdraws.
        5 if w.fault == 1 => (w.fault, w.pc[p]) = (2, 7),
        5 => w.pc[p] = 6,
        6 => super::enter_admit::admit(w, c),
        7 => super::enter_admit::withdraw(w, c, k),
        8 => return super::enter_wait::wait(w, c, k),
        9 => super::enter_wait::reask(w, c),
        _ => w.pc[p] = if pass(w, pc - 10, k, BY_WAIT) { pc + 1 } else { 8 },
    }
    true
}

// enter::enter's one claims hold: an entry standing answers by its stage,
// else the record decides, and Go pushes Making in the same hold
// (SplitAdmit: in a later one). The undone waits push over a pending wipe.
fn enter(w: &mut World, c: u8, k: Knob) {
    let p = usize::from(c) + 2;
    if let Some(e) = w.entries.iter().flatten().next().map(|e| over(e.stage)) {
        return match e {
            ClaimError::Resetting => again(w, c),
            _ => taken(w, c),
        };
    }
    let waits = matches!(k, Knob::ResetInMaking | Knob::WaitInMaking);
    match w.admits(c) {
        Admit::Go if k == Knob::SplitAdmit => w.pc[p] = 4,
        Admit::Go => push(w, c, k),
        Admit::Rearm => rearm(w, c),
        Admit::Resetting if waits => {
            push(w, c, k);
            w.pc[p] = 8;
        }
        Admit::Resetting => again(w, c),
        Admit::Refused => (w.events, w.pc[p]) = (w.events | REFUSED, OUT),
    }
}
