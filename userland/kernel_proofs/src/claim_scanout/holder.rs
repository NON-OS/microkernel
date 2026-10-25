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

//! bootfb's thread in the takeover model (L79), a round at a time: its claim
//! (the entry refused once the scanout was taken, then the admit with
//! at_admit and the hold, and the wait for a draw under way), its map and
//! store (holder_map.rs) and its give_back (give_back.rs).

use super::events::{BUSY, REFUSED_ENTER};
use super::knob::Knob;
use super::stage_rule::over;
use super::types::Stage;
use super::world::{code, stage, Entry};
use super::world_state::World;

pub(super) const ENTER: u8 = 0;
const ADMIT: u8 = 1;
pub(super) const QUIET: u8 = 2;
pub(super) const WITHDRAW: u8 = 3;
pub(super) const LOOKUP: u8 = 4;

pub(super) fn holder(w: &mut World, k: Knob) -> bool {
    let epoch = w.round + 1;
    match w.pc[2] {
        ENTER => enter(w, epoch),
        ADMIT => super::holder_admit::admit(w, epoch, k),
        // bootfb::admitted, the lock let go: a draw under way ends first.
        QUIET if w.draws > 0 => return false,
        QUIET => w.pc[2] = LOOKUP,
        WITHDRAW => {
            w.fb = w.fb.filter(|e| e.epoch != epoch);
            w.round_over();
        }
        pc if pc < super::give_back::RETIRE => super::holder_map::step(w, pc),
        pc => return super::give_back::step(w, pc),
    }
    true
}

/// enter's one hold: an entry standing answers by its stage (over); once the
/// scanout was taken bootfb's claim is refused; else Making is pushed.
fn enter(w: &mut World, epoch: u8) {
    if let Some(e) = w.fb {
        let _why = over(stage(e.stage));
        w.events |= BUSY;
        return w.round_over();
    }
    if w.gone {
        w.events |= REFUSED_ENTER;
        return w.round_over();
    }
    w.fb = Some(Entry { epoch, stage: code(Stage::Making), inflight: 0, leavers: 0 });
    w.pc[2] = ADMIT;
}
