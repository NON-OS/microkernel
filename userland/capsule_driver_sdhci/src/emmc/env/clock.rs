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

//! The millisecond clock, deadlines on it and a bounded pause.

/// A monotonic millisecond clock. Every wait in the engine is bounded by it,
/// never by a count of loop turns, so a fast CPU does not cut a wait short.
pub trait Clock {
    fn now_ms(&self) -> u64;
    /// Called once per turn of every polling loop.
    fn relax(&self) {
        core::hint::spin_loop();
    }
}

/// A deadline on a `Clock`.
#[derive(Clone, Copy, Debug)]
pub struct Deadline {
    end: u64,
}

impl Deadline {
    pub fn after<C: Clock>(clock: &C, ms: u64) -> Self {
        Self { end: clock.now_ms().saturating_add(ms) }
    }

    pub fn passed<C: Clock>(&self, clock: &C) -> bool {
        clock.now_ms() >= self.end
    }
}

/// Wait at least `ms` milliseconds. A millisecond clock read at an unknown
/// point inside its first tick can run up to one tick short, so the wait
/// runs one tick past `ms`.
pub fn pause<C: Clock>(clock: &C, ms: u64) {
    let start = clock.now_ms();
    while clock.now_ms().saturating_sub(start) <= ms {
        clock.relax();
    }
}
