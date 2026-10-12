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

//! Watching a join the driver runs on.

use super::{Machine, Phase, NOT_YET_MS, WATCH_EVERY_MS};

impl Machine {
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
