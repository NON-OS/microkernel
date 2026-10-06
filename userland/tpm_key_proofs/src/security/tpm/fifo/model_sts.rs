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

//! The model's status register: how it reads, and what a write does.

use super::model::{Model, State};
use super::regs::{STS_COMMAND_READY, STS_DATA_AVAIL, STS_EXPECT, STS_GO, STS_VALID};

impl Model {
    pub fn wanted(&self) -> Option<usize> {
        let header = self.received.get(2..6)?;
        let size = u32::from_be_bytes([header[0], header[1], header[2], header[3]]) as usize;
        Some(self.parsed_len.unwrap_or(size))
    }

    pub fn expecting(&self) -> bool {
        self.state == State::Reception && self.wanted().is_none_or(|w| self.received.len() < w)
    }

    pub fn status(&mut self) -> u8 {
        if self.state == State::Execution {
            self.exec_polls = self.exec_polls.saturating_sub(1);
            if self.exec_polls == 0 {
                self.state = State::Completion;
            }
        }
        let mut sts = STS_VALID;
        if self.state == State::Ready {
            sts |= STS_COMMAND_READY;
        }
        if self.expecting() {
            sts |= STS_EXPECT;
        }
        if self.state == State::Completion && self.pos < self.response.len() {
            sts |= STS_DATA_AVAIL;
        }
        sts
    }

    pub fn write_sts(&mut self, value: u8) {
        if !self.active {
            self.violations.push("status write without locality");
        }
        if value & STS_COMMAND_READY != 0 {
            self.received.clear();
            self.pos = 0;
            self.state = match self.state {
                State::Idle | State::Ready => State::Ready,
                _ => State::Idle,
            };
        }
        if value & STS_GO != 0 {
            if self.state == State::Reception && !self.expecting() {
                self.state = State::Execution;
                self.commands += 1;
                self.ran = self.received.clone();
            } else {
                self.violations.push("tpmGo before the command was complete");
            }
        }
    }
}
