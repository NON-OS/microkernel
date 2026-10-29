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

//! Starting the core. It is launched with no word, and its endpoint looked
//! for. A core with no profile yet exits before it listens (it reads its
//! name from a stdin it has not got), so after LOOK_MS the person is asked
//! for a name (start_name.rs). A busy launch is asked again; the endpoint is
//! looked for all along, and used whenever it answers.

pub use super::start_phase::{Phase, Step, EBUSY, EVERY_MS, LOOK_MS};

#[derive(Debug)]
pub struct Start {
    pub phase: Phase,
    last: i64,
}

impl Start {
    pub fn new() -> Self {
        Start { phase: Phase::Launch(None), last: i64::MIN }
    }

    /// The step at `now`: at most one every EVERY_MS.
    pub fn tick(&mut self, now: i64) -> Step {
        if matches!(self.phase, Phase::Up | Phase::Stopped(_))
            || now.saturating_sub(self.last) < EVERY_MS
        {
            return Step { launch: None, look: false };
        }
        self.last = now;
        if let Phase::Look { since, named } = self.phase {
            if now - since >= LOOK_MS {
                self.phase =
                    if named { Phase::Stopped("the core did not start") } else { Phase::Ask };
            }
        }
        let launch = if let Phase::Launch(w) = &self.phase { Some(w.clone()) } else { None };
        Step { launch, look: true }
    }

    /// MkAppLaunch answered `rc` at `now`.
    pub fn launched(&mut self, rc: i64, now: i64) {
        let Phase::Launch(w) = &self.phase else { return };
        self.phase = match rc {
            0 => Phase::Look { since: now, named: w.is_some() },
            EBUSY => return,
            _ => Phase::Stopped("the core could not be started"),
        };
    }

    /// The endpoint answered.
    pub fn found(&mut self) {
        self.phase = Phase::Up;
    }
}

impl Default for Start {
    fn default() -> Self {
        Start::new()
    }
}
