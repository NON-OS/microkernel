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

//! How often the poll loop asks the controller driver for one endpoint's
//! report. Every ask is an IPC round trip to driver.xhci0, and each endpoint
//! was asked at every pass of a 1 ms loop, a thousand times a second
//! whether anything was touched or not, which kept both capsules awake on
//! an idle machine. An endpoint is now asked no more often than the
//! controller serves it, and once quiet for a second no more than every
//! QUIET_PERIOD_MS, until it reports again.

/// An endpoint that has reported nothing for this long is quiet.
const QUIET_AFTER_MS: u64 = 1000;
/// How often a quiet endpoint is asked, as often as a full-speed keyboard
/// is served: its first report after a pause waits at most this long more.
pub const QUIET_PERIOD_MS: u64 = 8;
/// The longest the loop waits on its inbox, so key repeats and rescans are
/// looked at this often whatever is bound.
pub const MAX_WAIT_MS: u64 = 16;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Pace {
    period_ms: u64,
    due_ms: u64,
    heard_ms: u64,
}

impl Pace {
    /// An endpoint the controller serves every `period_ms`, due at once.
    pub fn new(period_ms: u64) -> Self {
        Self { period_ms: period_ms.max(1), due_ms: 0, heard_ms: 0 }
    }

    pub fn due(&self, now_ms: u64) -> bool {
        now_ms >= self.due_ms
    }

    /// The endpoint was asked at `now_ms`; `heard` if it had a report.
    pub fn asked(&mut self, now_ms: u64, heard: bool) {
        if heard {
            self.heard_ms = now_ms;
        }
        let quiet = now_ms.saturating_sub(self.heard_ms) >= QUIET_AFTER_MS;
        let period = if quiet { self.period_ms.max(QUIET_PERIOD_MS) } else { self.period_ms };
        self.due_ms = now_ms.saturating_add(period);
    }
}

/// How long the loop waits on its inbox: until the first endpoint is due,
/// at most MAX_WAIT_MS, and at least 1 ms, as a wait of 0 waits for good.
pub fn inbox_wait_ms<'a>(now_ms: u64, paces: impl Iterator<Item = &'a Pace>) -> u64 {
    let first_due = paces.map(|p| p.due_ms.saturating_sub(now_ms)).min();
    first_due.unwrap_or(MAX_WAIT_MS).clamp(1, MAX_WAIT_MS)
}
