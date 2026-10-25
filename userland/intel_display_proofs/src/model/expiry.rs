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

//! A deadline for the model's waits: expired after a number of checks.
//! The shipping deadline is a clock; in the model, time is the reads the
//! code makes, so a count is the model's clock.

use crate::engine::access::Expiry;
use std::cell::Cell;

pub struct Polls {
    left: Cell<u32>,
}

impl Polls {
    pub fn new(n: u32) -> Self {
        Polls { left: Cell::new(n) }
    }
}

impl Expiry for Polls {
    fn expired(&self) -> bool {
        let n = self.left.get();
        self.left.set(n.saturating_sub(1));
        n == 0
    }
}
