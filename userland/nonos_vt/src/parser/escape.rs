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

//! After ESC: an escape sequence, or the introducer of a longer one.

use super::handler::Handler;
use super::machine::{Parser, State};

impl Parser {
    pub(super) fn escape<H: Handler>(&mut self, h: &mut H, b: u8) {
        match b {
            0x00..=0x1F => h.execute(b),
            0x20..=0x2F => {
                self.seq.collect(b);
                self.state = State::EscapeInter;
            }
            b'[' => self.state = State::CsiEntry,
            b']' => self.start_string(State::Osc),
            b'P' => self.start_string(State::DcsEntry),
            b'X' | b'^' | b'_' => self.state = State::Ignored,
            0x30..=0x7E => {
                self.seq.final_byte = b;
                h.esc(&self.seq);
                self.state = State::Ground;
            }
            0x7F => {}
            // Bytes past 0x7F are not part of any escape sequence.
            _ => self.state = State::Ground,
        }
    }

    pub(super) fn escape_inter<H: Handler>(&mut self, h: &mut H, b: u8) {
        match b {
            0x00..=0x1F => h.execute(b),
            0x20..=0x2F => self.seq.collect(b),
            0x30..=0x7E => {
                self.seq.final_byte = b;
                if !self.seq.truncated() {
                    h.esc(&self.seq);
                }
                self.state = State::Ground;
            }
            0x7F => {}
            _ => self.state = State::Ground,
        }
    }

    pub(super) fn start_string(&mut self, state: State) {
        self.buf.clear();
        self.buf_full = false;
        self.state = state;
    }
}
