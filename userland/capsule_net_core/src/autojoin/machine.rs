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


//! When autojoin acts, kept free of the kernel and the driver so the host
//! proofs run it unchanged. The glue in `tick` makes the calls; this decides
//! which one is due and what each outcome leads to.
//!
//! A join takes the driver seconds (the hunt, then authentication and the
//! four-way handshake), and net_core's one loop serves every TCP, UDP, DHCP
//! and DNS client, so autojoin never waits for one: it hands the driver the
//! request with a short wait, and if no answer came in that time the join runs
//! on in the driver while autojoin watches the link once a second until it is
//! associated or `JOIN_WATCH_MS` has passed. Each saved network is tried at
//! most once per boot, so a wrong passphrase is not replayed at the access
//! point.

/// The longest any call autojoin makes from the serve loop waits for its
/// answer.
pub const CALL_MS: u64 = 25;
/// Waiting for the driver, the store or the policy restore.
pub const NOT_YET_MS: i64 = 3_000;
/// The driver is ready but no saved network was heard: scan again this much
/// later.
pub const RESCAN_MS: i64 = 15_000;
/// Passes with the driver ready and no saved network in range before autojoin
/// gives up for this boot.
pub const EMPTY_PASSES_MAX: u8 = 8;
/// How long a join handed to the driver is watched before it counts as failed:
/// past the driver's own bound (the hunt and its joins, at most 16 s, on the RTL8821CE),
/// with room for the access point's slowest handshake.
pub const JOIN_WATCH_MS: i64 = 25_000;
/// How often a running join's link is looked at.
pub const WATCH_EVERY_MS: i64 = 1_000;

/// How one pass went.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Step {
    /// Not yet possible: no policy, no driver, no store, or a driver not ready.
    NotYet,
    /// Ready, but no due saved network was heard.
    NoneInRange,
    /// Saved network `index` was handed to the driver and no answer came
    /// within `CALL_MS`; the join runs on in the driver.
    Started(u8),
    /// Saved network `index` was tried and the driver answered at once.
    Answered { index: u8, joined: bool },
    /// Nothing left to do this boot.
    Done,
}

/// What is due at a given time.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Due {
    Nothing,
    /// Run one pass (`Step`).
    Attempt,
    /// Look at the link of the join in progress.
    Watch,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Phase {
    Waiting,
    Joining { index: u8, until_ms: i64 },
    Done,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Machine {
    phase: Phase,
    next_ms: i64,
    tried: u8,
    empty_passes: u8,
}

impl Machine {
    pub const fn new() -> Self {
        Self { phase: Phase::Waiting, next_ms: 0, tried: 0, empty_passes: 0 }
    }

    /// The saved networks already tried, one bit each.
    pub fn tried(&self) -> u8 {
        self.tried
    }

    pub fn done(&self) -> bool {
        self.phase == Phase::Done
    }

    pub fn due(&self, now_ms: i64) -> Due {
        if now_ms < self.next_ms {
            return Due::Nothing;
        }
        match self.phase {
            Phase::Waiting => Due::Attempt,
            Phase::Joining { .. } => Due::Watch,
            Phase::Done => Due::Nothing,
        }
    }

    /// A link is already bound: leave it alone and look again later.
    pub fn bound(&mut self, now_ms: i64) {
        if self.phase == Phase::Waiting {
            self.next_ms = now_ms + NOT_YET_MS;
        }
    }

    pub fn after_attempt(&mut self, now_ms: i64, step: Step) {
        let wait = match step {
            Step::NotYet => NOT_YET_MS,
            Step::Answered { joined: true, .. } | Step::Done => {
                self.phase = Phase::Done;
                return;
            }
            Step::Answered { index, joined: false } => {
                self.tried |= bit(index);
                NOT_YET_MS
            }
            Step::Started(index) => {
                self.tried |= bit(index);
                self.phase = Phase::Joining { index, until_ms: now_ms + JOIN_WATCH_MS };
                WATCH_EVERY_MS
            }
            Step::NoneInRange => {
                self.empty_passes = self.empty_passes.saturating_add(1);
                if self.empty_passes >= EMPTY_PASSES_MAX {
                    self.phase = Phase::Done;
                    return;
                }
                RESCAN_MS
            }
        };
        self.next_ms = now_ms + wait;
    }

    /// What the link said while a join runs: `Some(true)` associated,
    /// `Some(false)` not (yet), `None` no answer in time (the driver is busy
    /// joining). Returns the index of a join that just ended, and whether it
    /// joined, for the log.
    pub fn after_watch(&mut self, now_ms: i64, associated: Option<bool>) -> Option<(u8, bool)> {
        let Phase::Joining { index, until_ms } = self.phase else { return None };
        if associated == Some(true) {
            self.phase = Phase::Done;
            return Some((index, true));
        }
        if now_ms >= until_ms {
            self.phase = Phase::Waiting;
            self.next_ms = now_ms + NOT_YET_MS;
            return Some((index, false));
        }
        self.next_ms = now_ms + WATCH_EVERY_MS;
        None
    }
}

impl Default for Machine {
    fn default() -> Self {
        Self::new()
    }
}

fn bit(index: u8) -> u8 {
    1u8.checked_shl(u32::from(index)).unwrap_or(0)
}
