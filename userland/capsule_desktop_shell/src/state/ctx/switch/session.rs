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

//! One run of Alt+Tab presses. It walks a copy of the order taken at its
//! first press, so each press moves on to the next window rather than back
//! to the one the last press left, though the focus it gave reorders the
//! live list. The run ends a hold after its last press.

use alloc::vec::Vec;

use super::order::Win;

/// How long the strip stays up after a press, in milliseconds: long enough
/// to read it and press again, short enough to be gone when typing resumes.
pub const SWITCH_HOLD_MS: i64 = 1200;

pub struct Session {
    pub(super) windows: Vec<Win>,
    pub(super) at: usize,
    /// The last press went back, Shift+Alt+Tab.
    pub(super) back: bool,
    pub(super) until_ms: i64,
}

impl Session {
    pub(super) fn new(windows: Vec<Win>, now_ms: i64) -> Self {
        let until_ms = now_ms.saturating_add(SWITCH_HOLD_MS);
        Session { windows, at: 0, back: false, until_ms }
    }

    /// Whether the run is still up at `now_ms`.
    pub fn live(&self, now_ms: i64) -> bool {
        now_ms < self.until_ms
    }

    /// When the run ends, for the serve loop's wake.
    pub fn until_ms(&self) -> i64 {
        self.until_ms
    }

    /// The windows the strip shows, and which one the run is on.
    pub fn shown(&self) -> (&[Win], usize) {
        (&self.windows, self.at % self.windows.len().max(1))
    }

    pub(super) fn step(&mut self, now_ms: i64, back: bool) -> Option<Win> {
        let n = self.windows.len();
        if n < 2 {
            return None;
        }
        let at = self.at % n;
        self.at = if back { (at + n - 1) % n } else { (at + 1) % n };
        self.back = back;
        self.until_ms = now_ms.saturating_add(SWITCH_HOLD_MS);
        Some(self.windows[self.at])
    }
}
