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

//! When the headphone jack is read: twice a second while the codec answers,
//! and at once after an unsolicited response. A codec that stopped answering
//! was asked twice a second forever, a 100 ms verb timeout each time. Now
//! each unanswered read in a row doubles the wait before the next (an answer
//! resets it), and after `MAX_FAILS` the codec is `Lost` and not asked again.

/// The wait between reads while the codec answers.
pub const POLL_MS: u64 = 500;
/// Unanswered reads in a row, their waits doubling 0.5 s to 16 s, to `Lost`.
pub const MAX_FAILS: u32 = 6;

/// The codec answering, this many reads in a row unanswered, or not read.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum JackState {
    Answering,
    Retrying(u32),
    Lost,
}

pub struct JackSchedule {
    last: u64,
    events: u32,
    fails: u32,
}

impl JackSchedule {
    pub const fn new(now: u64) -> Self {
        JackSchedule { last: now, events: 0, fails: 0 }
    }

    pub fn state(&self) -> JackState {
        match self.fails {
            0 => JackState::Answering,
            n if n < MAX_FAILS => JackState::Retrying(n),
            _ => JackState::Lost,
        }
    }

    /// The wait before the next read, after the run of unanswered reads.
    pub fn interval(&self) -> u64 {
        POLL_MS << self.fails.min(MAX_FAILS)
    }

    /// Whether to read the jack at `now`, `events` unsolicited responses in.
    pub fn due(&mut self, now: u64, events: u32) -> bool {
        let early = now.saturating_sub(self.last) < self.interval() && events == self.events;
        if early || self.state() == JackState::Lost {
            return false;
        }
        self.last = now;
        self.events = events;
        true
    }

    /// Note whether the read `due` called for was answered.
    pub fn answered(&mut self, ok: bool) {
        self.fails = if ok { 0 } else { self.fails.saturating_add(1) };
    }
}
