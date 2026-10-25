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

//! The model's clock: every reading moves it on 50 us, so a deadline
//! passes after a known number of polls; or it can be made unreadable.

use crate::engine::access::Clock;
use std::cell::Cell;

pub struct Ticks {
    pub now: Cell<u64>,
    pub broken: bool,
}

impl Ticks {
    pub fn new() -> Self {
        Ticks { now: Cell::new(0), broken: false }
    }
}

impl Clock for Ticks {
    fn now_us(&self) -> Option<u64> {
        if self.broken {
            return None;
        }
        self.now.set(self.now.get() + 50);
        Some(self.now.get())
    }
}
