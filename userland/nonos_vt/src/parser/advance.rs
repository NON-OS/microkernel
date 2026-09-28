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

//! One byte through the machine: the transitions every state shares, then
//! the state's own.

use super::handler::Handler;
use super::machine::{Parser, State};

impl Parser {
    pub fn advance<H: Handler>(&mut self, h: &mut H, b: u8) {
        match b {
            // CAN and SUB abandon whatever was being collected.
            0x18 | 0x1A => {
                self.flush_utf8(h);
                h.execute(b);
                self.state = State::Ground;
                return;
            }
            // ESC ends a string, dispatching it, and starts a new sequence.
            0x1B => {
                match self.state {
                    State::Osc => self.end_osc(h),
                    State::DcsPass => self.end_dcs(h),
                    _ => {}
                }
                self.flush_utf8(h);
                self.seq.clear();
                self.state = State::Escape;
                return;
            }
            _ => {}
        }
        match self.state {
            State::Ground => self.ground(h, b),
            State::Escape => self.escape(h, b),
            State::EscapeInter => self.escape_inter(h, b),
            State::CsiEntry | State::CsiParam | State::CsiInter | State::CsiIgnore => {
                self.csi_state(h, b)
            }
            State::DcsEntry | State::DcsParam | State::DcsInter | State::DcsIgnore => {
                self.dcs_state(b)
            }
            State::DcsPass => self.dcs_pass(b),
            State::Osc => self.osc_byte(h, b),
            State::Ignored => {}
        }
    }
}
