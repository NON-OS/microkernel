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

//! Whether a repeat is due at a tick.

use super::state::KeyRepeat;
use super::{DELAY_MS, LIMIT_MS, RATE_MS};

impl KeyRepeat {
    /// The key to press again at `now_ms`, if a repeat is due. A tick that
    /// comes late does not make up the repeats it missed in a burst.
    pub fn due(&mut self, now_ms: u64) -> Option<u8> {
        if self.key == 0 {
            return None;
        }
        let Some(since) = self.since_ms else {
            self.since_ms = Some(now_ms);
            self.next_ms = now_ms.saturating_add(DELAY_MS);
            return None;
        };
        if now_ms.saturating_sub(since) >= LIMIT_MS {
            *self = Self::new();
            return None;
        }
        if now_ms < self.next_ms {
            return None;
        }
        let behind = now_ms - self.next_ms;
        self.next_ms = if behind >= RATE_MS { now_ms + RATE_MS } else { self.next_ms + RATE_MS };
        Some(self.key)
    }
}
