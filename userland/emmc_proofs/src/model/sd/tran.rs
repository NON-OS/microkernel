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

//! Data transfer mode (4.3): CSD, select, status, bus width, block length,
//! reads and writes, and the CMD12 that ends an open-ended transfer.

use super::super::card::{Out, Reply, St};
use super::card::{out, APP_CMD, BLOCK_LEN_ERROR};
use super::SdCard;

/// R1 bit 22, ILLEGAL_COMMAND: an argument the command does not define.
const ILLEGAL_COMMAND: u32 = 1 << 22;

impl SdCard {
    pub(super) fn transfer(&mut self, app: bool, idx: u8, arg: u32, blocks: u16, now: u64) -> Out {
        let me = (arg >> 16) as u16 == self.cfg.rca;
        let before = self.status();
        match (app, idx, self.state) {
            (false, 9, St::Stby) if me => out(Reply::Long(self.csd())),
            (false, 7, St::Stby) if me => {
                self.state = St::Tran;
                out(Reply::Short(before))
            }
            (false, 13, _) if me => out(Reply::Short(before)),
            // ACMD6: 00b one line, 10b four (4.10.4).
            (true, 6, St::Tran) => match arg & 3 {
                0 | 2 => {
                    self.width = if arg & 3 == 2 { 4 } else { 1 };
                    out(Reply::Short(before | APP_CMD))
                }
                _ => out(Reply::Short(before | APP_CMD | ILLEGAL_COMMAND)),
            },
            // A high capacity card's blocks are 512 bytes whatever CMD16 says.
            (false, 16, St::Tran) if !self.cfg.ccs && arg != 512 => {
                out(Reply::Short(before | BLOCK_LEN_ERROR))
            }
            (false, 16, St::Tran) => out(Reply::Short(before)),
            (false, 17 | 18 | 24 | 25, St::Tran) => self.rw(idx, arg, blocks, before),
            (false, 12, St::Data | St::Rcv) => {
                self.end(now);
                out(Reply::Short(before))
            }
            _ => out(Reply::Silent),
        }
    }
}
