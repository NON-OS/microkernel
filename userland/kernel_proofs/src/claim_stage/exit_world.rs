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

//! An exit racing its claim's own users (L75): a grant path inside the claim
//! (a bind or a map), a sibling's release and a newer claimant waiting for the
//! device, every order tried; the kernel's stage rule included by path. `new`:
//! the exit counts as a release and the last one out removes the entry; the
//! old exit removed the entry first and stopped the device if none named it.

use super::stage_rule::gone;
use super::types::Stage;

/// A claim: epoch, stage (as `code`), releases under way, paths inside.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub(super) struct Entry(pub u64, pub u8, pub u32, pub u32);

/// Paths: 0 the grant path, 1 the exit, 2 the sibling's release, 3 the newer
/// claimant, 4 a kill landing where the release waits (L77). `bad`: an old
/// path wrote a newer claim's device, or frames were freed under Bus Master.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub(super) struct World {
    pub entry: Option<Entry>,
    pub grant: bool,
    pub bus_on: bool,
    pub bad: bool,
    pub yields: bool,
    pub steps: [u8; 5],
}

const STAGES: [Stage; 5] =
    [Stage::Making, Stage::Ready, Stage::Leaving, Stage::Orphaned, Stage::Exiting];

pub(super) fn code(s: Stage) -> u8 {
    STAGES.iter().position(|&t| t == s).unwrap_or(0) as u8
}

impl World {
    /// The old claim's stage, while its entry stands.
    pub(super) fn ours(&self) -> Option<Stage> {
        self.entry.filter(|e| e.0 == 1).map(|e| STAGES[e.1 as usize])
    }
    pub(super) fn write(&mut self) {
        self.bad |= self.entry.is_some_and(|e| e.0 == 7);
    }
    pub(super) fn stop(&mut self) {
        self.write();
        self.bus_on = false;
    }
    pub(super) fn free(&mut self) {
        self.bad |= self.grant && self.bus_on;
        self.grant = false;
    }
    /// Change the old claim's entry; under the new rule it goes once gone.
    pub(super) fn edit(&mut self, new: bool, f: impl FnOnce(&mut Entry, Stage)) {
        let Some(s) = self.ours() else { return };
        let Some(mut e) = self.entry else { return };
        f(&mut e, s);
        let left = gone(STAGES[e.1 as usize], e.2, e.3);
        self.entry = (!(new && left)).then_some(e);
    }
}
