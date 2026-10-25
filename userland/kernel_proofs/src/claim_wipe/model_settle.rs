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

//! release.rs's settle: one release of the claim at (pid, epoch) done; the
//! entry goes with the last of them, and its wipe is owed in that same hold
//! (gone.rs `remove`), so no claim begins between the two.

use super::model_stage::stage_rule::gone;
use super::model_stage::types::Stage;
use super::world::World;

/// release.rs `named`: the claim at (p, epoch), being released.
pub(super) fn named(w: &World, p: usize, epoch: u64) -> bool {
    let leaving = |s: Stage| matches!(s, Stage::Leaving | Stage::Exiting);
    w.entry.is_some_and(|e| e.pid == p && e.epoch == epoch && leaving(e.stage))
}

/// release.rs `settle`: true when this was the last release and the entry
/// went, its wipe owed in the same hold (gone.rs `remove`).
pub(super) fn settle(w: &mut World, p: usize, epoch: u64) -> bool {
    if !named(w, p, epoch) {
        return false;
    }
    let Some(e) = w.entry.as_mut() else { return false };
    e.leavers = e.leavers.saturating_sub(1);
    if !gone(e.stage, e.leavers, 0) {
        return false;
    }
    w.entry = None;
    w.owe(epoch);
    true
}
