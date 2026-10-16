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

//! One SD command and its response. The command sits in SD_CMD0 to SD_CMD4
//! and the response lands there too (SD_CMD0 holds the index field, 3Fh
//! for R3), or in the ping-pong buffer for R2: 3Fh, then the register's
//! bits 127:8. Every command sent is noted, card or not; only a powered
//! card in the slot answers.

use super::card_cmd::Reply;
use super::reader::Reader;
use crate::regs::sd::{PPBUF_BASE2, SD_CMD0, SD_STAT1};

impl Reader {
    pub(super) fn command(&mut self) -> bool {
        let index = self.reg(SD_CMD0) & 0x3F;
        let a: Vec<u8> = (1..=4).map(|i| self.reg(SD_CMD0 + i)).collect();
        let arg = u32::from_be_bytes([a[0], a[1], a[2], a[3]]);
        self.sent.push(index);
        if self.knobs.pull_on_cmd.take_if(|&mut i| i == index).is_some() {
            self.pull();
        }
        let powered = self.powered();
        let Some(card) = self.slot.as_mut().filter(|_| powered) else { return false };
        match card.command(index, arg) {
            Reply::Silent => return false,
            Reply::None => {}
            Reply::Short(field, word) => {
                self.regs.insert(SD_CMD0, field);
                for (i, b) in word.to_be_bytes().into_iter().enumerate() {
                    self.regs.insert(SD_CMD0 + 1 + i as u16, b);
                }
            }
            Reply::Long(words) => {
                let bytes: Vec<u8> = words.iter().flat_map(|w| w.to_be_bytes()).collect();
                self.regs.insert(PPBUF_BASE2, 0x3F);
                for (i, &b) in bytes.iter().take(15).enumerate() {
                    self.regs.insert(PPBUF_BASE2 + 1 + i as u16, b);
                }
            }
        }
        self.regs.insert(SD_STAT1, 0);
        true
    }
}
