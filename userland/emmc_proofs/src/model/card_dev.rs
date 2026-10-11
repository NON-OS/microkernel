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

//! The eMMC device as the host model drives it.

use super::card::{Card, Out, St};
use super::card_flags::RESP_FLAGS;
use super::dev::Dev;

impl Dev for Card {
    fn power(&mut self, on: bool) {
        Card::power(self, on)
    }

    fn command(&mut self, idx: u8, arg: u32, host_blocks: u16, now: u64) -> Out {
        Card::command(self, idx, arg, host_blocks, now)
    }

    fn end(&mut self, now: u64) {
        Card::end(self, now)
    }

    fn take_write(&mut self, lba: u64, data: &[u8], now: u64) {
        Card::take_write(self, lba, data, now)
    }

    fn max_hz(&self) -> u32 {
        match self.state {
            St::Idle | St::Ready | St::Ident | St::Off | St::Inactive => 400_000,
            _ if self.hs => 52_000_000,
            _ => 26_000_000,
        }
    }

    fn count(&self) -> Option<u16> {
        self.count
    }

    fn width(&self) -> u8 {
        self.width
    }

    fn corrupts(&self, host_width: u8) -> bool {
        self.cfg.broken8 && host_width == 8
    }

    /// A host cannot tell an eMMC device from an SD card before it asks, so
    /// in idle state SD's CMD8 (R7) and CMD55 are commands a driver may
    /// send: an eMMC device leaves them unanswered.
    fn flags(&self, idx: u8) -> Option<u16> {
        match (idx, self.state) {
            (8 | 55, St::Idle) => Some(0x1a),
            _ => RESP_FLAGS.iter().find(|(i, _)| *i == idx).map(|&(_, f)| f),
        }
    }

    fn carries_data(&self, idx: u8) -> bool {
        matches!(idx, 8 | 17 | 18 | 24 | 25)
    }
}
