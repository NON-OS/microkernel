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

//! What the card answers each command with (SD Physical Layer 4.7.4 and
//! 4.9), for the commands this driver sends. One it does not know, or one
//! addressed to another card, gets no answer, as on the bus.

use super::card::Card;
use super::linux::*;

pub enum Reply {
    /// The command takes no response (CMD0).
    None,
    /// A 48-bit response: the index field, then 32 bits.
    Short(u8, u32),
    /// R2: the CID or CSD.
    Long([u32; 4]),
    Silent,
}

impl Card {
    pub fn command(&mut self, index: u8, arg: u32) -> Reply {
        let app = std::mem::take(&mut self.app);
        let mine = arg >> 16 == self.rca as u32;
        match (app, index) {
            (_, 0) => {
                self.reset();
                Reply::None
            }
            (_, 8) if self.v2 => Reply::Short(8, arg & 0xFFF),
            (_, 55) => {
                self.app = true;
                Reply::Short(55, self.status() | APP_CMD)
            }
            (true, 41) => self.op_cond(arg),
            (_, 2) if self.state == READY => self.then(IDENT, Reply::Long(self.cid)),
            (_, 3) => {
                (self.rca, self.state) = (0x4567, STBY);
                Reply::Short(3, (self.rca as u32) << 16 | self.status() & 0x1E00)
            }
            (_, 9) if mine => Reply::Long(self.csd),
            (_, 7) if mine => self.then(TRAN, Reply::Short(7, self.status())),
            (_, 16) | (true, 6) => {
                Reply::Short(index, self.status() | if app { APP_CMD } else { 0 })
            }
            (_, 13) if mine => Reply::Short(13, self.poll()),
            (_, 17 | 18 | 24 | 25) if self.state == TRAN => self.start_data(index, arg),
            (_, 12) => {
                let next = if self.programming > 0 { PRG } else { TRAN };
                self.then(next, Reply::Short(12, self.status()))
            }
            _ => Reply::Silent,
        }
    }
}
