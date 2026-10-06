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

//! Collecting the parameters of a CSI sequence, up to its final byte.

use super::handler::Handler;
use super::machine::{Parser, State};

impl Parser {
    pub(super) fn csi_state<H: Handler>(&mut self, h: &mut H, b: u8) {
        let st = self.state;
        match b {
            0x00..=0x1F => h.execute(b),
            0x7F => {}
            0x40..=0x7E => {
                if st != State::CsiIgnore && !self.seq.truncated() {
                    self.seq.final_byte = b;
                    h.csi(&self.seq);
                }
                self.state = State::Ground;
            }
            _ if st == State::CsiIgnore => {}
            0x30..=0x39 if st != State::CsiInter => {
                self.seq.params.digit(b - b'0');
                self.state = State::CsiParam;
            }
            b';' | b':' if st != State::CsiInter => {
                self.seq.params.separator(b == b':');
                self.state = State::CsiParam;
            }
            // A private marker is only a marker as the first byte.
            0x3C..=0x3F if st == State::CsiEntry => {
                self.seq.private = b;
                self.state = State::CsiParam;
            }
            0x20..=0x2F => {
                self.seq.collect(b);
                self.state = State::CsiInter;
            }
            // Anything else out of place: read to the final byte, then drop.
            _ => self.state = State::CsiIgnore,
        }
    }
}
