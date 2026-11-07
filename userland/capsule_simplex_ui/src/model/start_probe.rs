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

//! A look that ran out (start.rs): the core is asked for again with the
//! word it was last given. MkAppLaunch answers busy while a Linux run goes
//! on, one at a time, so a busy answer to that launch is a run not ended:
//! a slow start, waited for up to WAIT_MS from the launch that started it.
//! An answer of 0 says the core had ended without listening and was
//! started again (start.rs launched): a core with no name is then asked
//! one, and one with a name is stopped on after MAX_STARTS starts.

use super::start::{Phase, Start, DID_NOT_START, LOOK_MS, MAX_STARTS, WAIT_MS};

impl Start {
    /// The step a tick at `now` takes past a look or a probe that ran out.
    pub(super) fn ran_out(&mut self, now: i64) {
        match self.phase {
            Phase::Look { since, named } if now.saturating_sub(since) >= LOOK_MS => {
                self.phase = match named && self.starts >= MAX_STARTS {
                    true => Phase::Stopped(DID_NOT_START),
                    false => {
                        self.probe = true;
                        Phase::Launch(self.word.clone())
                    }
                };
            }
            Phase::Launch(_) if self.probe && now.saturating_sub(self.started) >= WAIT_MS => {
                self.probe = false;
                self.phase = Phase::Stopped(DID_NOT_START);
            }
            _ => {}
        }
    }
}
