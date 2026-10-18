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

//! The mouse reports a pad sends with no touch report among them. On a pad
//! that cannot read its mode back, a run of GONE_REPORTS of them, GONE_MS
//! from its first, says it has left touchpad mode (input/poll/pointer.rs).
//! One or two between touch reports are a pointing stick or the driver's
//! own stall, and a gap with none at all is a still finger.

use super::recover::now_ms;

const GONE_REPORTS: u8 = 4;
const GONE_MS: u64 = 100;

#[derive(Clone, Copy, Default)]
pub struct MouseRun {
    count: u8,
    first_ms: u64,
}

impl MouseRun {
    /// A touch report ends the run.
    pub(super) fn touched(&mut self) {
        self.count = 0;
    }

    /// A mouse report: whether the run it adds to says the pad left
    /// touchpad mode.
    pub fn mouse(&mut self) -> bool {
        let now = now_ms();
        if self.count == 0 {
            self.first_ms = now;
        }
        self.count = self.count.saturating_add(1);
        self.count >= GONE_REPORTS && now.saturating_sub(self.first_ms) >= GONE_MS
    }
}
