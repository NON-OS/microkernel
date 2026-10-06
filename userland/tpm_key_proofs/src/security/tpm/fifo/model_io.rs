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

//! The model's data port, and the bookkeeping behind its violations.

use super::model::{Model, State};

impl Model {
    /// A command byte arriving through the FIFO.
    pub fn take(&mut self, value: u8) {
        self.spend();
        if self.state == State::Ready {
            self.state = State::Reception;
        }
        if !self.expecting() {
            self.wrong("command byte the part was not expecting");
        }
        self.received.push(value);
    }

    /// One FIFO access, charged against the burst count last read.
    pub fn spend(&mut self) {
        if !self.active {
            self.violations.push("fifo access without locality");
        }
        match self.budget.checked_sub(1) {
            Some(left) => self.budget = left,
            None => self.violations.push("more bytes than the burst count allowed"),
        }
    }

    pub fn wrong(&mut self, what: &'static str) -> u8 {
        self.violations.push(what);
        u8::MAX
    }
}
