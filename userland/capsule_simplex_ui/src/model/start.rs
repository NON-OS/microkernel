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
//! name from a stdin it has not got). A core that has not listened after
//! LOOK_MS is asked for again (start_probe.rs): a busy answer says its run
//! is still going, a slow first start, and it is waited for; an answer of 0
//! says it ended, so the person is asked for a name (start_name.rs) when it
//! had none, and it is started again when it had one. A busy launch is
//! asked again; the endpoint is looked for all along, and used whenever it
//! answers. A channel lost once up starts it all again (start_lost.rs).

use alloc::string::String;

pub use super::start_phase::{
    Phase, Step, DID_NOT_START, EBUSY, EVERY_MS, LOOK_MS, MAX_STARTS, WAIT_MS,
};

#[derive(Debug)]
pub struct Start {
    pub phase: Phase,
    pub(super) last: i64,
    /// Launched again after a lost channel: the core has its profile, so a
    /// core that does not listen is stopped on, not asked a name.
    pub(super) again: bool,
    /// Channels lost in a row with no frame from the core between them.
    pub(super) losses: u8,
    /// The word the run going was started with, asked again by a probe.
    pub(super) word: Option<String>,
    /// The launch asked is a look that ran out, asking whether the run goes on.
    pub(super) probe: bool,
    /// Launches answered 0 since the name was given or the channel lost.
    pub(super) starts: u8,
    /// When the last launch answered 0.
    pub(super) started: i64,
}

impl Start {
    pub fn new() -> Self {
        Start {
            phase: Phase::Launch(None),
            last: i64::MIN,
            again: false,
            losses: 0,
            word: None,
            probe: false,
            starts: 0,
            started: 0,
        }
    }

    /// The step at `now`: at most one every EVERY_MS.
    pub fn tick(&mut self, now: i64) -> Step {
        if matches!(self.phase, Phase::Up | Phase::Stopped(_))
            || now.saturating_sub(self.last) < EVERY_MS
        {
            return Step { launch: None, look: false };
        }
        self.last = now;
        self.ran_out(now);
        let launch = if let Phase::Launch(w) = &self.phase { Some(w.clone()) } else { None };
        Step { launch, look: true }
    }

    /// MkAppLaunch answered `rc` at `now`.
    pub fn launched(&mut self, rc: i64, now: i64) {
        let Phase::Launch(w) = &self.phase else { return };
        let (named, word) = (w.is_some() || self.again, w.clone());
        // A busy answer leaves the probe asking; any other ends it.
        let probe = rc != EBUSY && core::mem::take(&mut self.probe);
        self.phase = match rc {
            EBUSY => return,
            0 if probe && !named => Phase::Ask,
            0 => {
                (self.word, self.started) = (word, now);
                self.starts = self.starts.saturating_add(1);
                Phase::Look { since: now, named }
            }
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
