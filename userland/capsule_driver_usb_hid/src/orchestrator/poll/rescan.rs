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

//! When the poll loop looks at the ports again: every RESCAN_MS while no
//! report comes in, for a device plugged in or pulled out.

/// The ports wait this long after the last report, and between rescans.
pub const RESCAN_MS: u64 = 64;

pub struct Rescan {
    next_ms: u64,
}

impl Rescan {
    /// The ports were just looked at, at `now_ms`.
    pub fn new(now_ms: u64) -> Self {
        Self { next_ms: now_ms.saturating_add(RESCAN_MS) }
    }

    /// A report came in: the devices are in use, the rescan waits.
    pub fn heard(&mut self, now_ms: u64) {
        self.next_ms = self.next_ms.max(now_ms.saturating_add(RESCAN_MS));
    }

    /// Whether every port is looked at now; if so the next rescan is due
    /// RESCAN_MS on.
    pub fn rescan(&mut self, now_ms: u64) -> bool {
        let due = now_ms >= self.next_ms;
        if due {
            self.next_ms = now_ms.saturating_add(RESCAN_MS);
        }
        due
    }
}
