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

//! Collecting the parameters of a DCS sequence, up to its final byte.

use super::machine::{Parser, State};

impl Parser {
    pub(super) fn dcs_state(&mut self, b: u8) {
        let st = self.state;
        match b {
            0x00..=0x1F | 0x7F => {}
            0x40..=0x7E => {
                self.seq.final_byte = b;
                self.state = if st == State::DcsIgnore { State::DcsIgnore } else { State::DcsPass };
            }
            _ if st == State::DcsIgnore => {}
            0x30..=0x39 if st != State::DcsInter => {
                self.seq.params.digit(b - b'0');
                self.state = State::DcsParam;
            }
            b';' | b':' if st != State::DcsInter => {
                self.seq.params.separator(b == b':');
                self.state = State::DcsParam;
            }
            0x3C..=0x3F if st == State::DcsEntry => {
                self.seq.private = b;
                self.state = State::DcsParam;
            }
            0x20..=0x2F => {
                self.seq.collect(b);
                self.state = State::DcsInter;
            }
            _ => self.state = State::DcsIgnore,
        }
    }
}
