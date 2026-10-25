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

//! ask.rs and held.rs: what the claims ask of the record, each in a claims
//! hold. `owe` and `rearm` also note, as ground truth, when the record began
//! to be pending and that a new pass of reset writes begins (I4, I5).

use super::admit_rule::{admits, Admit};
use super::due_rule::pending;
use super::holder_rule::{same, Name};
use super::model_end::unsettled;
use super::model_enter::KEEP_CAP;
use super::phase_rule::Phase;
use super::world::World;
use super::world_knob::Knob;
use super::world_types::{Truth, Wipe};

impl World {
    /// ask.rs `owe`, in the hold that removes the claim's entry.
    pub(super) fn owe(&mut self, epoch: u64) {
        let now = self.now;
        let held = |r: &&mut Wipe| r.epoch == epoch && r.phase == Phase::Held;
        let Some(r) = self.rec.as_mut().filter(held) else { return };
        r.phase = Phase::Owed { since: now };
        self.pending += 1;
        self.truth = Truth { owed_at: now, since: now, writes: 0, ..self.truth };
    }

    /// ask.rs `admits`, for process `p`: the kept bound asked once the
    /// record is read (L80); no tenancy let in with it reached (I10).
    pub(super) fn admits(&mut self, p: usize) -> Admit {
        let (rec, past, pr) = (self.rec, self.kept_bytes() >= KEEP_CAP, &mut self.procs[p]);
        pr.over_cap = if self.knob == Knob::CapEarly { pr.over_cap } else { past };
        let same = same(rec.and_then(|r| r.by), pr.name);
        let a = admits(rec.map(|r| r.phase), same, self.now, pr.over_cap);
        if a == Admit::Go && self.kept_bytes() + unsettled(self) >= KEEP_CAP {
            self.bad(10, "a tenancy let in past the kept frames' bound (I10)");
        }
        a
    }

    /// ask.rs `rearm`: a lost function asked once more.
    pub(super) fn rearm(&mut self) {
        let lost = |r: &&mut Wipe| matches!(r.phase, Phase::Lost { .. });
        let Some(r) = self.rec.as_mut().filter(lost) else { return };
        r.phase = Phase::Recheck;
        self.pending += 1;
        self.truth = Truth { since: self.now, writes: 0, rearmed: true, ..self.truth };
    }

    /// held.rs `held`: the claim's record, replacing a kept or lost one;
    /// false only over a wipe still under way.
    pub(super) fn held(&mut self, epoch: u64, by: Option<Name>) -> bool {
        if self.rec.is_some_and(|r| pending(r.phase)) {
            return false;
        }
        let (phase, method, marker, saved) = (Phase::Held, None, None, None);
        self.rec =
            Some(Wipe { epoch, by, phase, method, marker, saved, bme_off: false, busy: false });
        true
    }
}
