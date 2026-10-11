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

//! READ and WRITE, single and multiple block, in the card's addressing.

use super::super::card::{Out, Phase, Reply, St};
use super::card::{out, ADDRESS_ERROR, OUT_OF_RANGE};
use super::SdCard;

impl SdCard {
    /// A multi-block transfer runs until CMD12: the host's Block Count is
    /// how many blocks the card sends or takes.
    pub(super) fn rw(&mut self, idx: u8, arg: u32, blocks: u16, before: u32) -> Out {
        let n = if idx == 17 || idx == 24 { 1 } else { blocks as u64 };
        let lba = if self.cfg.ccs {
            arg as u64
        } else if arg.is_multiple_of(512) {
            arg as u64 / 512
        } else {
            return out(Reply::Short(before | ADDRESS_ERROR));
        };
        if lba + n > self.capacity() {
            return out(Reply::Short(before | OUT_OF_RANGE));
        }
        if idx == 17 || idx == 18 {
            self.state = St::Data;
            let bytes = (0..n).flat_map(|i| self.block(lba + i)).collect();
            return Out { reply: Reply::Short(before), phase: Phase::Read(bytes), busy_ms: 0 };
        }
        self.state = St::Rcv;
        Out { reply: Reply::Short(before), phase: Phase::Write { lba }, busy_ms: 0 }
    }
}
