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

//! The object behind a timerfd.
//!
//! Like an eventfd's counter it is the object, not the descriptor: dup, fork
//! and every thread reach one timer through their own descriptors, so it is
//! kept with the family's shared objects and a descriptor names it by index.
//! Times are on the guest's monotonic clock, in milliseconds.

#[derive(Clone, Copy, Default)]
pub struct Timer {
    /// When it next fires; zero while disarmed.
    pub due: u64,
    /// How often it fires after that; zero for once.
    pub every: u64,
    /// The clock it was made on, which an absolute time is read against.
    pub clock: u64,
}

impl Timer {
    pub fn fired(&self, now: u64) -> bool {
        self.due != 0 && now >= self.due
    }

    /// How many times it has fired by `now`, moving it past them: a
    /// one-shot timer disarms, a periodic one steps to its next time.
    pub fn take(&mut self, now: u64) -> u64 {
        if !self.fired(now) {
            return 0;
        }
        if self.every == 0 {
            self.due = 0;
            return 1;
        }
        let times = 1 + (now - self.due) / self.every;
        /*
         * Saturating: an interval the guest set near the top of the clock
         * would otherwise wrap the next time round to one already past,
         * and the timer would fire on every look for ever.
         */
        self.due = self.due.saturating_add(times.saturating_mul(self.every));
        times
    }
}
