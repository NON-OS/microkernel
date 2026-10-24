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
//! A clock that moves a fixed step on every read, so every wait ends, and
//! one that cannot be read at all.

use core::cell::Cell;
use nonos_intel_display_hw::access::Clock;

pub struct Steps {
    now: Cell<u64>,
    step: u64,
}

impl Steps {
    pub fn new(step: u64) -> Self {
        Steps { now: Cell::new(0), step }
    }

    pub fn now(&self) -> u64 {
        self.now.get()
    }
}

impl Clock for Steps {
    fn now_us(&self) -> Option<u64> {
        let t = self.now.get();
        self.now.set(t + self.step);
        Some(t)
    }
}

/// A clock whose reads all fail.
pub struct Dead;

impl Clock for Dead {
    fn now_us(&self) -> Option<u64> {
        None
    }
}
