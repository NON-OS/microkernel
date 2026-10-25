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

//! A claim's last step in the enter model (L78): pending::admit, which writes
//! the claim's Held record and makes its entry Ready in one hold, or
//! pending::withdraw, which takes the entry back and nothing else.

use super::enter_check::{I1, I1_LOST, I7};
use super::enter_events::WITHDRAWN;
use super::enter_parts::{Entry, Knob, IN, OUT};
use super::enter_record::service;
use super::enter_stage::Stage;
use super::enter_world::World;
use super::phase_rule::LOST_HOLD;

/// pending::admit: Held written, refused over a pending record, and the
/// entry Ready. I1: the device holds no other service's state, and no Lost
/// hold is still running.
pub(super) fn admit(w: &mut World, c: u8) {
    let (i, by) = (usize::from(c), service(c));
    let making = w.entries[i].filter(|e| e.stage == Stage::Making);
    let Some(e) = making.filter(|e| w.held(e.epoch, by)) else {
        w.pc[i + 2] = 7;
        return;
    };
    if w.dirty.is_some_and(|s| s != by) {
        w.fail(I1);
    }
    if w.lost.is_some_and(|t| w.now < t.saturating_add(LOST_HOLD)) {
        w.fail(I1_LOST);
    }
    w.entries[i] = Some(Entry { stage: Stage::Ready, ..e });
    (w.dirty, w.pc[i + 2]) = (Some(by), IN);
}

/// pending::withdraw: the entry removed in one hold (the bus master stop
/// before it and the detach after change nothing here); HeldAtEnter also
/// drops the record its enter wrote. I7: the record is as the push found it.
pub(super) fn withdraw(w: &mut World, c: u8, k: Knob) {
    let i = usize::from(c);
    let mine = w.entries[i].take().map(|e| e.epoch);
    if k == Knob::HeldAtEnter && w.rec.map(|r| r.epoch) == mine {
        w.rec = None;
    }
    if w.rec != w.snap[i] {
        w.fail(I7);
    }
    (w.events, w.pc[i + 2]) = (w.events | WITHDRAWN, OUT);
}
