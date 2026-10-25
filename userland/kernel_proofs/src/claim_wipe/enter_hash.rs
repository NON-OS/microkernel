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

//! Worlds of the enter model hashed for the seen-set (L78). The kernel's
//! phase, method, end and stage types carry no hash of their own, so each
//! is folded to plain numbers here; equal worlds give equal keys.

use std::hash::{Hash, Hasher};

use super::enter_parts::Rec;
use super::enter_world::World;
use super::phase_rule::Phase;

type Key = (u8, u8, (u8, u64), Option<u8>, bool, bool);

fn phase(p: Phase) -> (u8, u64) {
    match p {
        Phase::Held => (0, 0),
        Phase::Owed { since } => (1, since),
        Phase::Flr { at } => (2, at),
        Phase::D3 { at } => (3, at),
        Phase::D0 { at } => (4, at),
        Phase::Recheck => (5, 0),
        Phase::Kept => (6, 0),
        Phase::Lost { at } => (7, at),
    }
}

fn rec(r: Option<Rec>) -> Option<Key> {
    r.map(|r| (r.epoch, r.by, phase(r.phase), r.method.map(|m| m as u8), r.marker, r.busy))
}

impl Hash for World {
    fn hash<H: Hasher>(&self, h: &mut H) {
        self.entries.map(|e| e.map(|e| (e.epoch, e.stage as u8, e.leavers))).hash(h);
        (rec(self.rec), self.job.map(|j| (rec(Some(j.rec)), j.end.map(|e| e as u8), j.now)))
            .hash(h);
        (self.snap.map(rec), self.count, self.now, self.epoch, self.pc, self.tries).hash(h);
        (self.dirty, self.lost, self.marker, self.written, self.late, self.fault).hash(h);
        (self.killed, self.prey, self.events, self.bad).hash(h);
        (self.fate.flr, self.fate.obeys, self.fate.back, self.fate.display).hash(h);
    }
}
