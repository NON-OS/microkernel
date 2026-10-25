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

//! When a turn has sat in one step long enough to name it, pure: the
//! watchdog thread asks every WATCH_MS and says what this returns. Waiting
//! for requests is never a stall. Each stuck step is said once, and once
//! more when the loop goes on, at most STALL_LINES lines a boot.

pub use super::stall_step::{Seen, Step};

pub const STUCK_MS: i64 = 2000;
pub const STALL_LINES: u8 = 8;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Say {
    /// Stuck in this step this long.
    Stuck(Seen, i64),
    /// The stuck turn went on, `ms` after it was first seen stuck.
    WentOn(u32, i64),
}

#[derive(Default)]
pub struct StallRule {
    stuck: Option<(Seen, i64)>,
    said: u8,
}

impl StallRule {
    pub fn check(&mut self, s: Seen, now: i64) -> Option<Say> {
        if self.said >= STALL_LINES {
            return None;
        }
        let out = match self.stuck {
            Some((was, at)) if (was.turn, was.step) != (s.turn, s.step) => {
                self.stuck = None;
                Some(Say::WentOn(was.turn, now - at))
            }
            Some(_) => None,
            None if s.step != Step::Wait && now - s.since >= STUCK_MS => {
                self.stuck = Some((s, now));
                Some(Say::Stuck(s, now - s.since))
            }
            None => None,
        };
        self.said += out.is_some() as u8;
        out
    }
}
