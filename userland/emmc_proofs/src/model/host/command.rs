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

//! A write to the Command register: the command recorded, held to the
//! host's rules, and answered when the bus is powered and clocked.

use super::sim::Sim;
use super::types::{CmdRec, Issued};

impl Sim {
    pub(super) fn command(&mut self, word: u16) {
        self.put(0x0e, 2, word as u32);
        let index = ((word >> 8) & 0x3f) as u8;
        let arg = self.get(0x08, 4);
        let mode = self.get(0x0c, 2) as u16;
        let blocks = self.get(0x06, 2) as u16;
        let blksz = self.get(0x04, 2) as u16;
        let now = self.now();
        self.cmds.push(CmdRec {
            index,
            arg,
            word,
            mode,
            blocks,
            blksz,
            hz: self.sd_hz(),
            width: self.host_width(),
            hs: self.regs[0x28] & 0x04 != 0,
            at: now,
        });
        self.check_issue(index, word);
        if self.cfg.swap_at_cmd == Some(self.cmds.len()) {
            self.cfg.swap_at_cmd = None;
            self.take_card();
            self.put_sd(super::super::sd::SdCfg::default());
        }
        let hz = self.sd_hz();
        if self.regs[0x29] & 1 == 0 || hz == 0 {
            self.int |= 1 << 15 | 1 << 16;
            return;
        }
        let limit = self.dev_ref().max_hz();
        if hz > limit {
            self.violate(format!("CMD{index} at {hz} Hz, the card's limit {limit}"));
        }
        self.answer(Issued { index, arg, mode, blocks, blksz, word, now });
    }
}
