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

//! The card's data: a read or write command sets where the data phase
//! starts, the phase moves whole blocks, a single-block one ends on its
//! own and a multi-block one at CMD12, and a write leaves the card
//! programming for a few CMD13 polls.

use super::card::Card;
use super::card_cmd::Reply;
use super::linux::{PRG, RCV, TRAN};

/// SD state 5, sending data.
const DATA: u32 = 5;

impl Card {
    pub(super) fn start_data(&mut self, index: u8, arg: u32) -> Reply {
        let status = self.status();
        self.at = if self.high_capacity { arg as u64 } else { arg as u64 / 512 };
        self.multi = matches!(index, 18 | 25);
        self.state = if index < 24 { DATA } else { RCV };
        Reply::Short(index, status)
    }

    /// The block at `lba` as the card holds it.
    pub fn block(&self, lba: u64) -> Vec<u8> {
        self.data.get(&lba).cloned().unwrap_or_else(|| vec![0; 512])
    }

    pub(super) fn read_out(&mut self, len: usize) -> Option<Vec<u8>> {
        let end = self.at.checked_add(len as u64 / 512).filter(|&e| e <= self.blocks)?;
        if self.state != DATA {
            return None;
        }
        let out = (self.at..end).flat_map(|b| self.block(b)).collect();
        self.at = end;
        if !self.multi {
            self.state = TRAN;
        }
        Some(out)
    }

    pub(super) fn write_in(&mut self, bytes: &[u8]) -> bool {
        let end = (self.at + (bytes.len() / 512) as u64).min(self.blocks);
        if self.state != RCV || end - self.at != (bytes.len() / 512) as u64 {
            return false;
        }
        for (i, chunk) in bytes.chunks(512).enumerate() {
            self.data.insert(self.at + i as u64, chunk.to_vec());
        }
        self.at = end;
        self.programming = self.program_polls;
        if !self.multi {
            self.state = PRG;
        }
        true
    }
}
