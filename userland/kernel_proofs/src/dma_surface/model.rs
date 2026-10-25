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

//! A run's teardown on the model: teardown.rs asks grant_run.rs `hold`,
//! which runs grant_detach.rs `detach`, and frees the run when not held.

use super::common::{foreign, Orphan, Registry};

impl Registry {
    /// True when the registry keeps `run`; the broker frees it otherwise.
    pub fn release(&mut self, run: (u64, u64)) -> bool {
        self.detach(run);
        let held = self.orphans.iter().any(|o| o.grant == Some(run));
        if held {
            self.runs.push(run);
        } else {
            self.given_back.push(run);
        }
        held
    }

    /// Every live surface over `run` loses its frames: one another process
    /// maps becomes an orphan holding the run, any other is released.
    fn detach(&mut self, run: (u64, u64)) {
        let attaches = self.attaches.clone();
        let mut gone = Vec::new();
        for (i, s) in self.slots.iter_mut().enumerate() {
            if s.grant != Some(run) || s.frames.is_empty() {
                continue;
            }
            let frames = core::mem::take(&mut s.frames);
            if foreign(&attaches, s.handle, s.owner) {
                let (owner, handle, free) = (s.owner, s.handle, Vec::new());
                self.orphans.push(Orphan { owner, handle, frames, free, grant: Some(run) });
            } else {
                gone.push(i);
            }
        }
        gone.into_iter().rev().for_each(|i| drop(self.slots.remove(i)));
    }
}
