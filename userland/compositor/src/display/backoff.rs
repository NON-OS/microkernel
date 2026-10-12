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

//! While on GOP, which display name to try next and when (section 12):
//! one name a tick, 500 ms apart, the gap doubling after each full round
//! of the list that found none, up to 30 s. A machine with no display
//! driver stops asking twice a second, and one whose driver comes up late
//! still gets it within one round of the gap. A present the kernel refuses
//! because bootfb holds the framebuffer starts the list again at once.

pub const FIRST_MS: i64 = 500;
pub const LAST_MS: i64 = 30_000;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Backoff {
    /// The index into the display list tried next.
    pub next: usize,
    pub gap_ms: i64,
    pub due_ms: i64,
}

impl Backoff {
    pub fn new(now_ms: i64) -> Self {
        Backoff { next: 0, gap_ms: FIRST_MS, due_ms: now_ms.saturating_add(FIRST_MS) }
    }

    /// The name to try now, if one is due, with the next one scheduled.
    pub fn due(&mut self, now_ms: i64, names: usize) -> Option<usize> {
        if names == 0 || now_ms < self.due_ms {
            return None;
        }
        let i = self.next % names;
        self.next = (i + 1) % names;
        if self.next == 0 {
            self.gap_ms = self.gap_ms.saturating_mul(2).min(LAST_MS);
        }
        self.due_ms = now_ms.saturating_add(self.gap_ms);
        Some(i)
    }

    /// The kernel says a driver holds the boot framebuffer (a present
    /// answered EBUSY): ask again from the first name, now, at the first gap.
    pub fn held(&mut self, now_ms: i64) {
        *self = Backoff { next: 0, gap_ms: FIRST_MS, due_ms: now_ms };
    }
}
