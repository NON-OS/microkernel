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

//! The world of the enter model (L78): A holding the function, its release
//! and exit, the wipe its going owes, B (another service) and A2 (A's own,
//! restarted) claiming it, the device as it truly is, and the clock.

use super::enter_fates::Fate;
use super::enter_parts::{Entry, Job, Rec, END, TRIES};
use super::enter_stage::Stage;
use super::phase_rule::Phase;

/// Paths by `pc`: 0 A's give_back, 1 A's exit, 2 the tick, 3 B, 4 A2, 5 the
/// clock, 6 a fault in a claim's middle steps, 7 a kill of claimant `prey`.
/// Claimants 1 (B) and 2 (A2), and A as 0, index `entries`, `tries` and
/// `snap` (the record as each pushed Making, for I7). `dirty`: the service
/// whose state the device holds; `lost`: when it was last found Lost;
/// `marker`: the bit as the device holds it; `written`: its last reset;
/// `late`: the path whose owe waits for its own next step (OweApart).
#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub(super) struct World {
    pub entries: [Option<Entry>; 3],
    pub rec: Option<Rec>,
    pub job: Option<Job>,
    pub count: u8,
    pub now: u64,
    pub epoch: u8,
    pub pc: [u8; 8],
    pub tries: [u8; 3],
    pub snap: [Option<Rec>; 3],
    pub dirty: Option<u8>,
    pub lost: Option<u64>,
    pub marker: bool,
    pub written: Option<u64>,
    pub late: Option<u8>,
    pub fault: u8,
    pub killed: bool,
    pub prey: u8,
    pub events: u16,
    pub fate: Fate,
    pub bad: Option<&'static str>,
}

/// A holding the function Ready over its Held record; which of A's release
/// and exit, B and A2 take part.
pub(super) fn start(fate: Fate, release: bool, exit: bool, b: bool, a2: bool) -> World {
    let on = |x: bool| if x { 0 } else { END };
    let a = Entry { epoch: 1, stage: Stage::Ready, leavers: 0 };
    let rec = Rec { epoch: 1, by: 0, phase: Phase::Held, method: None, marker: false, busy: false };
    World {
        entries: [Some(a), None, None],
        rec: Some(rec),
        epoch: 2,
        pc: [on(release), on(exit), 0, on(b), on(a2), 0, 0, on(b || a2)],
        tries: [0, TRIES, TRIES],
        dirty: Some(0),
        prey: if b { 1 } else { 2 },
        fate,
        ..World::default()
    }
}
