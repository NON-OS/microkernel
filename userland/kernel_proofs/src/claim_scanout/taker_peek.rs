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

//! The takeover's loop in the model (L79): bootfb's claim read, and the
//! kernel's own `next` deciding what comes after.

use super::knob::Knob;
use super::scanout_rule::{next, Next};
use super::taker::{ADMIT, ENDS};
use super::types::Stage;
use super::world::stage;
use super::world_state::World;

/// One read of bootfb's claim under the table's lock (scanout::peek) and
/// what `next` makes of it. A release under way is spun on, which is waiting
/// here: the step cannot move until the stage changes.
pub(super) fn peek(w: &mut World, g: usize, k: Knob) -> bool {
    let seen = w.fb.map(|e| stage(e.stage));
    let leaving = seen == Some(Stage::Leaving) && k == Knob::LeavingDone;
    match if leaving { Next::Done } else { next(seen) } {
        Next::Wait => return false,
        Next::TakeBack => {
            w.g[g].seen = w.fb.map_or(0, |e| e.stage);
            w.pc[g] = super::take_order::RETIRE;
        }
        Next::Done => {
            w.g[g].late = false;
            w.pc[g] = if k == Knob::TakeAfterAdmit { ENDS } else { ADMIT };
        }
    }
    true
}
