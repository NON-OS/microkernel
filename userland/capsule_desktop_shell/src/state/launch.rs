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

//! A Launchpad launch the installer is still working on. The load reads the
//! app's artifacts out of the store and runs the whole verify and attestation
//! before the installer answers, which takes seconds; the shell used to wait
//! for that answer inside its frame loop, up to 30 s with nothing drawn. The
//! call now gets a short budget, and a load still running when it ends is
//! followed from here, on the shell's own turns, until the app shows up or the
//! installer is known to have finished without it.
//!
//! The installer's late answer cannot be read: it lands in the shell's reply
//! inbox, where the next call the shell makes drops it as not its own. Two
//! things stand in for it. The app's service is in the registry from the
//! moment the kernel spawns it, so it is looked for on the shell's turns. And
//! the installer serves one request at a time, in order, so once a health
//! probe sent after the load is answered, the load is over: if no app came of
//! it by then, it failed.
//!
//! It is timed on uptime. It was timed on the wall clock, which reads an
//! error until the RTC is read and is stepped back by NTP: a launch started
//! while it read the error, or before a step back, was never due again, so
//! it neither found its app nor failed, and every later Launchpad launch of
//! an installed app was told to wait for it, for the rest of the session.

use alloc::vec::Vec;

use crate::state::Uptime;

/// The most a launch is followed: the budget the blocking call had.
pub const DEADLINE_MS: i64 = 30_000;
/// The first probe, then twice the gap each time up to the cap. Every probe
/// the installer is too busy to answer keeps a place in its reply queue until
/// it does, and the kernel gives one caller eight such places; this schedule
/// holds six at most over the whole deadline, the load's own one beside them.
pub const FIRST_PROBE_MS: i64 = 500;
pub const PROBE_GAP_MAX_MS: i64 = 8_000;
/// How often the registry is asked for the app.
pub const LOOK_GAP_MS: i64 = 50;

/// What the shell should do on this turn.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Step {
    /// Nothing yet; look again on a later turn.
    Wait,
    /// Ask the installer whether it has finished the load.
    Probe,
    /// The app is up: hand it its focus frame.
    Started(u32),
    /// The installer finished without the app, or the deadline passed.
    Failed,
}

pub struct Launch {
    pub name: Vec<u8>,
    /// The shell's children before the load, so a new one can be told apart.
    pub children: Vec<u32>,
    started_ms: i64,
    next_probe_ms: i64,
    probe_gap_ms: i64,
    next_look_ms: i64,
    served: bool,
}

impl Launch {
    pub fn new(name: &[u8], children: Vec<u32>, now: Uptime) -> Self {
        let now_ms = now.0;
        Launch {
            name: name.to_vec(),
            children,
            started_ms: now_ms,
            next_probe_ms: now_ms.saturating_add(FIRST_PROBE_MS),
            probe_gap_ms: FIRST_PROBE_MS,
            next_look_ms: now_ms,
            served: false,
        }
    }

    /// Whether this turn should look for the app at all: the shell turns once
    /// per message it serves, and the registry need not be asked that often.
    pub fn due(&mut self, now: Uptime) -> bool {
        let now_ms = now.0;
        if now_ms < self.next_look_ms {
            return false;
        }
        self.next_look_ms = now_ms.saturating_add(LOOK_GAP_MS);
        true
    }

    /// The next step, given the app's pid if it has appeared.
    pub fn step(&self, now: Uptime, found: Option<u32>) -> Step {
        if let Some(pid) = found {
            return Step::Started(pid);
        }
        if self.served || self.elapsed_ms(now) >= DEADLINE_MS {
            return Step::Failed;
        }
        if now.0 >= self.next_probe_ms {
            return Step::Probe;
        }
        Step::Wait
    }

    /// What a probe found. Answered, the load is over and the next step
    /// settles it; unanswered, the next probe waits twice as long.
    pub fn probed(&mut self, now: Uptime, answered: bool) {
        let now_ms = now.0;
        if answered {
            self.served = true;
            return;
        }
        self.probe_gap_ms = self.probe_gap_ms.saturating_mul(2).min(PROBE_GAP_MAX_MS);
        self.next_probe_ms = now_ms.saturating_add(self.probe_gap_ms);
    }

    /// Whether the installer is known to have finished the load.
    pub fn served(&self) -> bool {
        self.served
    }

    /// How long the launch has run, which bounds the age of a child it made.
    pub fn elapsed_ms(&self, now: Uptime) -> i64 {
        now.0.saturating_sub(self.started_ms)
    }
}
