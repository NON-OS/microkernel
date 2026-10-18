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

//! The card's answer to a command: its response in the registers, then its
//! data phase or busy, or the timeout when it said nothing.

use super::super::card::{Phase, Reply};
use super::sim::Sim;
use super::types::{Issued, Xfer};

impl Sim {
    pub(super) fn answer(&mut self, c: Issued) {
        let Issued { index, arg, mode, blocks, blksz, word, now } = c;
        let data = word & 0x20 != 0;
        let out = self.dev().command(index, arg, blocks, now);
        match out.reply {
            // A command that expects no response completes whether or not
            // anything is listening.
            Reply::Silent if word & 3 == 0 => {}
            Reply::Silent => {
                self.int |= 1 << 15 | 1 << 16;
                return;
            }
            Reply::Nothing => {}
            Reply::Short(r) => self.put(0x10, 4, self.lied(index, r)),
            Reply::Long(v) => {
                let reg = v >> 8;
                for i in 0..4 {
                    self.put(0x10 + 4 * i, 4, (reg >> (32 * i)) as u32);
                }
            }
        }
        self.int |= 1;
        match out.phase {
            Phase::None => {
                if data {
                    // The card sends nothing: the host times out the data.
                    self.int |= 1 << 15 | 1 << 20;
                } else if word & 3 == 3 {
                    self.tc_at = Some(now + out.busy_ms);
                    self.poll_tc();
                }
            }
            Phase::Read(bytes) => {
                self.data(Xfer { index, mode, blocks, blksz, card_reads: true, bytes, lba: 0 })
            }
            Phase::Write { lba } => self.data(Xfer {
                index,
                mode,
                blocks,
                blksz,
                card_reads: false,
                bytes: Vec::new(),
                lba,
            }),
        }
    }
}
