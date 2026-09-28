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

//! How ITIMER_REAL moves on once it fires: a periodic one past every period
//! that ended unseen, a one-shot one stops. Pure, so the arithmetic is checked
//! without a guest.

use super::sigtimer::Itimer;

impl Itimer {
    /// Move past every period that has ended by `now`; false for a one-shot.
    pub fn rearm(&mut self, now: u64) -> bool {
        if self.interval == 0 {
            return false;
        }
        while self.due <= now {
            self.due = self.due.saturating_add(self.interval);
        }
        true
    }
}
