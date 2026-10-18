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

//! The two bounds of one wait, started and restarted by the clock.

use super::{Heard, ANON_QUIET_MS, ANON_TOTAL_MS, DIRECT_QUIET_MS, DIRECT_TOTAL_MS};

#[derive(Clone, Copy, Debug)]
pub struct Wait {
    quiet_ms: i64,
    quiet_until: i64,
    total_until: i64,
}

impl Wait {
    pub fn new(now_ms: i64, anonymous: bool) -> Self {
        let (quiet_ms, total_ms) = if anonymous {
            (ANON_QUIET_MS, ANON_TOTAL_MS)
        } else {
            (DIRECT_QUIET_MS, DIRECT_TOTAL_MS)
        };
        Wait {
            quiet_ms,
            quiet_until: now_ms.saturating_add(quiet_ms),
            total_until: now_ms.saturating_add(total_ms),
        }
    }

    /// Bytes arrived: the quiet bound starts again from now.
    pub fn heard(&mut self, now_ms: i64) {
        self.quiet_until = now_ms.saturating_add(self.quiet_ms);
    }

    pub fn check(&self, now_ms: i64) -> Heard {
        if now_ms >= self.total_until {
            return Heard::Total;
        }
        if now_ms >= self.quiet_until {
            return Heard::Quiet;
        }
        Heard::Waiting
    }
}
