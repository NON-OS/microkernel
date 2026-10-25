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

//! The card status, programming's end, and the blocks the card holds.

use super::super::card::St;
use super::card::READY_FOR_DATA;
use super::SdCard;

impl SdCard {
    /// CURRENT_STATE in 12:9 and READY_FOR_DATA, as R1 carries them.
    pub fn status(&self) -> u32 {
        let code = match self.state {
            St::Off | St::Idle | St::Inactive => 0,
            St::Ready => 1,
            St::Ident => 2,
            St::Stby => 3,
            St::Tran => 4,
            St::Data => 5,
            St::Rcv => 6,
            St::Prg => 7,
        };
        code << 9 | if self.state == St::Tran { READY_FOR_DATA } else { 0 }
    }

    pub fn tick(&mut self, now: u64) {
        if self.state == St::Prg && now >= self.busy_until {
            self.state = St::Tran;
        }
    }

    /// The data phase is over: all blocks moved, or CMD12.
    pub fn end(&mut self, now: u64) {
        match self.state {
            St::Data => self.state = St::Tran,
            St::Rcv => (self.state, self.busy_until) = (St::Prg, now + self.cfg.prg_ms),
            _ => {}
        }
    }

    pub fn take_write(&mut self, lba: u64, data: &[u8], now: u64) {
        for (i, chunk) in data.chunks(512).enumerate() {
            self.store.insert(lba + i as u64, chunk.try_into().expect("512-byte blocks"));
        }
        self.writes += 1;
        self.end(now);
    }

    /// A block never written reads as a pattern of its address.
    pub fn block(&self, lba: u64) -> [u8; 512] {
        self.store.get(&lba).copied().unwrap_or_else(|| {
            core::array::from_fn(|i| (lba as u8).wrapping_mul(29).wrapping_add(i as u8))
        })
    }
}
