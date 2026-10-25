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

//! The wipe record's questions as the claims ask them under their lock, in
//! the kernel's order (ask.rs, held.rs), for the enter model (L78).

use super::admit_rule::{admits, Admit};
use super::due_rule::pending;
use super::enter_parts::Rec;
use super::enter_world::World;
use super::holder_rule::{name_of, same};
use super::phase_rule::Phase;

/// The service a claimant serves: B's is 1, A's and A2's 0.
pub(super) fn service(c: u8) -> u8 {
    u8::from(c == 1)
}

impl World {
    pub(super) fn pending(&self) -> bool {
        self.rec.is_some_and(|r| pending(r.phase))
    }
    /// ask::owe: the Held record of `epoch` owed, and counted.
    pub(super) fn owe(&mut self, epoch: u8) {
        let since = self.now;
        if let Some(r) = self.rec.as_mut().filter(|r| r.epoch == epoch && r.phase == Phase::Held) {
            (r.phase, self.count) = (Phase::Owed { since }, self.count + 1);
        }
    }
    /// ask::rearm: a Lost record asked once more, and counted.
    pub(super) fn rearm(&mut self) {
        if let Some(r) = self.rec.as_mut().filter(|r| matches!(r.phase, Phase::Lost { .. })) {
            (r.phase, self.count) = (Phase::Recheck, self.count + 1);
        }
    }
    /// held::held: the claim's record, refused over a pending one.
    pub(super) fn held(&mut self, epoch: u8, by: u8) -> bool {
        let rec = Rec { epoch, by, phase: Phase::Held, method: None, marker: false, busy: false };
        let ok = !self.pending();
        self.rec = if ok { Some(rec) } else { self.rec };
        ok
    }
    /// ask::admits for claimant `c`; its kept frames are never over their
    /// bound here (the seeded model's).
    pub(super) fn admits(&self, c: u8) -> Admit {
        let name = |s: u8| name_of([if s == 0 { "driver.nic0" } else { "driver.other0" }]);
        let same = same(self.rec.and_then(|r| name(r.by)), name(service(c)));
        admits(self.rec.map(|r| r.phase), same, self.now, false)
    }
}
