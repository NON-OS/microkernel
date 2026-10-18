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

//! A function on a fake clock that records what the reset asked of it.

use super::reset_rule::{Flr, ResetIo};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub(super) enum Step {
    Pending(u64),
    Save,
    Initiate(Flr),
    Pause(u64),
    Ready(u64),
    Restore,
}

pub(super) struct Fake {
    pub(super) offers: Option<Flr>,
    pub(super) pending_until: u64,
    pub(super) answers_after: u64,
    pub(super) now: u64,
    pub(super) reset_at: Option<u64>,
    pub(super) steps: Vec<Step>,
}

impl Fake {
    pub(super) fn new(offers: Option<Flr>, pending_until: u64, answers_after: u64) -> Self {
        Fake { offers, pending_until, answers_after, now: 0, reset_at: None, steps: Vec::new() }
    }
}

impl ResetIo for Fake {
    fn flr(&mut self) -> Option<Flr> {
        self.offers
    }
    fn pending(&mut self, _: Flr) -> bool {
        self.steps.push(Step::Pending(self.now));
        self.now < self.pending_until
    }
    fn save(&mut self) {
        self.steps.push(Step::Save);
    }
    fn initiate(&mut self, how: Flr) {
        self.reset_at = Some(self.now);
        self.steps.push(Step::Initiate(how));
    }
    fn ready(&mut self) -> bool {
        self.steps.push(Step::Ready(self.now));
        self.now >= self.reset_at.unwrap() + self.answers_after
    }
    fn restore(&mut self) {
        self.steps.push(Step::Restore);
    }
    fn now_ms(&mut self) -> u64 {
        self.now
    }
    fn pause_ms(&mut self, ms: u64) {
        self.steps.push(Step::Pause(ms));
        self.now += ms;
    }
}
