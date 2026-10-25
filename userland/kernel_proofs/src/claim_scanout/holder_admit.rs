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

//! bootfb's admit in the takeover model (L79).

use super::events::REFUSED_ADMIT;
use super::holder::{QUIET, WITHDRAW};
use super::knob::Knob;
use super::scanout_rule::at_admit;
use super::taker::ADMIT as TAKER_ADMIT;
use super::types::Stage;
use super::world::{code, stage};
use super::world_state::World;

/// pending::admit for bootfb: at_admit asked with its own entry's stage; made
/// ready, the hold set in the same hold of the lock. Control asks nothing.
pub(super) fn admit(w: &mut World, epoch: u8, k: Knob) {
    let making = w.fb.filter(|e| e.epoch == epoch && stage(e.stage) == Stage::Making);
    let Some(e) = making else {
        w.pc[2] = WITHDRAW;
        return;
    };
    if k != Knob::Control && at_admit(true, w.gone, false, Some(stage(e.stage))).is_err() {
        w.events |= REFUSED_ADMIT;
        w.pc[2] = WITHDRAW;
        return;
    }
    w.edit(epoch, |e| e.stage = code(Stage::Ready));
    w.held = true;
    for g in (0..2).filter(|&g| w.pc[g] == TAKER_ADMIT) {
        w.g[g].late = true;
    }
    w.pc[2] = QUIET;
}
