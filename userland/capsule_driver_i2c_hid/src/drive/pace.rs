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

//! When the loop next reads the pad or looks at its interrupt line. Each
//! read is a transfer the controller driver waits out on the bus, and they
//! went every 2 ms whether a finger was on the pad or not: five hundred a
//! second on an idle machine. A pad that has sent nothing for
//! QUIET_AFTER_MS is now read every QUIET_PERIOD_MS until it reports again.

/// The read period while reports come: four reads to a pad's 8 ms report.
pub const ACTIVE_PERIOD_MS: u64 = 2;
/// A pad that has reported nothing for this long is quiet.
pub const QUIET_AFTER_MS: u64 = 1_000;
/// How often a quiet pad is read: its first report after a pause waits at
/// most this much longer, under one frame of a 60 Hz screen.
pub const QUIET_PERIOD_MS: u64 = 16;
/// The longest the loop waits on its inbox, whatever is bound.
pub const MAX_WAIT_MS: u64 = 500;

#[derive(Clone, Copy, Default)]
pub struct Pace {
    due_ms: u64,
    heard_ms: u64,
}

impl Pace {
    pub fn due(&self, now_ms: u64) -> bool {
        now_ms >= self.due_ms
    }

    /// The pad was read, or its line looked at, at `now_ms`; `heard` when a
    /// fresh report came.
    pub fn read(&mut self, now_ms: u64, heard: bool) {
        if heard {
            self.heard_ms = now_ms;
        }
        let quiet = now_ms.saturating_sub(self.heard_ms) >= QUIET_AFTER_MS;
        let period = if quiet { QUIET_PERIOD_MS } else { ACTIVE_PERIOD_MS };
        self.due_ms = now_ms.saturating_add(period);
    }

    /// Nothing is due for `ms` from `now_ms`.
    pub fn after(&mut self, now_ms: u64, ms: u64) {
        self.due_ms = now_ms.saturating_add(ms);
    }

    /// How long the loop waits on its inbox: until the next pass is due, at
    /// least 1 ms, as a wait of 0 waits for good, and at most MAX_WAIT_MS.
    pub fn inbox_wait_ms(&self, now_ms: u64) -> u64 {
        self.due_ms.saturating_sub(now_ms).clamp(1, MAX_WAIT_MS)
    }
}
