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

//! DDC/CI's timing, kept per output. A monitor needs 40 ms after a Get
//! VCP request before its reply can be read, and 50 ms after a message
//! before it takes the next; nothing is sent inside that quiet window,
//! and a caller waits it out (50 ms at most). A reading, or the refusal
//! the monitor gave, is answered from memory for a second, so asking over
//! and over costs the bus nothing.

use super::error::DdcError;
use super::reply::Reading;
use super::vcp::ALLOWED;

pub const REPLY_US: u64 = 40_000;
pub const QUIET_US: u64 = 50_000;
pub const KEEP_US: u64 = 1_000_000;

#[derive(Clone, Copy)]
pub struct Pacer {
    quiet_until: u64,
    kept: [Option<(Result<Reading, DdcError>, u64)>; ALLOWED.len()],
}

impl Pacer {
    pub const fn new() -> Pacer {
        Pacer { quiet_until: 0, kept: [None; ALLOWED.len()] }
    }

    /// When the next message may go: never later than QUIET_US from `now`,
    /// so a clock that stepped back cannot hold the bus for longer.
    pub fn next_at(&self, now: u64) -> u64 {
        self.quiet_until.min(now.saturating_add(QUIET_US))
    }

    /// A message went at `now`.
    pub fn sent(&mut self, now: u64) {
        self.quiet_until = now.saturating_add(QUIET_US);
    }

    pub fn kept(&self, code: u8, now: u64) -> Option<Result<Reading, DdcError>> {
        let (r, at) = (*self.kept.get(slot(code)?)?)?;
        (now >= at && now - at < KEEP_US).then_some(r)
    }

    pub fn keep(&mut self, code: u8, r: Option<Result<Reading, DdcError>>, now: u64) {
        if let Some(s) = slot(code).and_then(|i| self.kept.get_mut(i)) {
            *s = r.map(|r| (r, now));
        }
    }
}

impl Default for Pacer {
    fn default() -> Pacer {
        Pacer::new()
    }
}

fn slot(code: u8) -> Option<usize> {
    ALLOWED.iter().position(|c| *c == code)
}
