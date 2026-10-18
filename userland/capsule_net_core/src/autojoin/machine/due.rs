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

//! What the machine says is due, and leaving a bound link alone.

use super::{Due, Machine, Phase, NOT_YET_MS};

impl Machine {
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
}
