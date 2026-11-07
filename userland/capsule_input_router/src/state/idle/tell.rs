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

//! The word to the compositor: sent until it took it, a refused one again
//! after `RETRY_MS`.

use super::{Idle, RETRY_MS};

impl Idle {
    /// Whether the compositor must be told `want` now: it has not taken it,
    /// and it was not tried, or tried a retry's time ago. A changed word
    /// goes at once: a wake never waits on a blank's retry.
    pub fn due(&self, want: bool, now_ms: i64) -> bool {
        let again = match self.tried {
            Some((w, at)) if w == want => now_ms.saturating_sub(at) >= RETRY_MS,
            _ => true,
        };
        self.told != Some(want) && again
    }

    /// The compositor's answer to `want`, tried at `now_ms`.
    pub fn answered(&mut self, want: bool, now_ms: i64, took: bool) {
        self.tried = Some((want, now_ms));
        if took {
            self.told = Some(want);
        }
    }
}
