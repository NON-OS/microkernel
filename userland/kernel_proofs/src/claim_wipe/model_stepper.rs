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

//! A `step_all` call in flight (model_step), whether it waits for the table
//! another holds, and `due` as the steps ask it.

use super::due_rule::{due, Due};
use super::end_rule::End;
use super::phase_rule::{Phase, MS};
use super::world::World;
use super::world_knob::Knob;
use super::world_types::Wipe;

#[derive(Clone, Copy, Debug)]
pub(super) struct Stepper {
    pub wait: bool,
    pub stage: u8,
    pub now: u64,
    pub job: Option<Wipe>,
    pub end: Option<End>,
    /// Its end's kept frames settled (model_end).
    pub settled: bool,
}

impl Stepper {
    pub(super) const fn new(wait: bool) -> Self {
        Stepper { wait, stage: 0, now: 0, job: None, end: None, settled: false }
    }
}

/// `due`, but undo check 5 reads an FLR's Vendor ID a millisecond early.
pub(super) fn due_at(k: Knob, p: Phase, now: u64) -> Due {
    let early = k == Knob::VendorAt99 && matches!(p, Phase::Flr { .. });
    due(p, now + if early { MS } else { 0 })
}

/// A stage that takes the table waits while another holds it.
pub(super) fn blocked(w: &World, s: &Stepper, me: usize) -> bool {
    ((s.stage == 1 && s.wait) || s.stage == 5) && w.wipes_by.is_some_and(|by| by != me)
}
