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

//! Notifications held to a rate: more than `LIMIT` in a second and the
//! session stops reading them for `QUIET_MS`, then reads every connector
//! once, which is all a flood could have told it. A PPM that notifies
//! without end costs a bounded share of the capsule's time.

pub const WINDOW_MS: u64 = 1000;
pub const LIMIT: u32 = 32;
pub const QUIET_MS: u64 = 2000;

#[derive(Clone, Copy, Debug, Default)]
pub struct Storm {
    window_start: u64,
    count: u32,
    quiet_until: u64,
    rescan: bool,
    /// Storms seen, for the log.
    pub storms: u32,
}

impl Storm {
    /// Whether this notification is read.
    pub fn admit(&mut self, now: u64) -> bool {
        if now < self.quiet_until {
            return false;
        }
        if now.saturating_sub(self.window_start) >= WINDOW_MS {
            (self.window_start, self.count) = (now, 0);
        }
        self.count += 1;
        if self.count > LIMIT {
            self.quiet_until = now.saturating_add(QUIET_MS);
            (self.rescan, self.storms) = (true, self.storms + 1);
            return false;
        }
        true
    }

    /// True once, when a quiet spell has ended: every connector is read.
    pub fn take_rescan(&mut self, now: u64) -> bool {
        let due = self.rescan && now >= self.quiet_until;
        self.rescan &= !due;
        due
    }

    pub fn quiet(&self, now: u64) -> bool {
        now < self.quiet_until
    }
}
