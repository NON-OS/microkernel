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

//! A millisecond clock and deadlines on it. Waits are bounded by time,
//! never by a count of loop passes.

pub trait Clock {
    fn now_ms(&self) -> u64;
    /// Called once per pass of every polling loop; may sleep.
    fn idle(&self);
}

#[derive(Clone, Copy)]
pub struct Deadline(u64);

impl Deadline {
    pub fn after<C: Clock>(c: &C, ms: u64) -> Self {
        Deadline(c.now_ms().saturating_add(ms))
    }

    pub fn passed<C: Clock>(&self, c: &C) -> bool {
        c.now_ms() >= self.0
    }
}

/// True once `done` holds, false if `ms` pass first; `done` is asked once
/// more at the deadline, so an answer during the last pass is not lost.
pub fn until<C: Clock>(c: &C, ms: u64, mut done: impl FnMut() -> bool) -> bool {
    let d = Deadline::after(c, ms);
    loop {
        if done() {
            return true;
        }
        if d.passed(c) {
            return done();
        }
        c.idle();
    }
}

/// At least `ms` milliseconds.
pub fn hold<C: Clock>(c: &C, ms: u64) {
    let d = Deadline::after(c, ms.saturating_add(1));
    while !d.passed(c) {
        c.idle();
    }
}
