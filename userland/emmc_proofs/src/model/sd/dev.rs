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

//! The SD card as the host model drives it.

use super::super::card::{Out, St};
use super::super::dev::Dev;
use super::flags::FLAGS;
use super::SdCard;

impl Dev for SdCard {
    fn power(&mut self, on: bool) {
        SdCard::power(self, on)
    }

    fn command(&mut self, idx: u8, arg: u32, host_blocks: u16, now: u64) -> Out {
        SdCard::command(self, idx, arg, host_blocks, now)
    }

    fn end(&mut self, now: u64) {
        SdCard::end(self, now)
    }

    fn take_write(&mut self, lba: u64, data: &[u8], now: u64) {
        SdCard::take_write(self, lba, data, now)
    }

    /// 400 kHz until the card has an address, then Default Speed's 25 MHz.
    fn max_hz(&self) -> u32 {
        match self.state {
            St::Off | St::Idle | St::Ready | St::Ident | St::Inactive => 400_000,
            _ => 25_000_000,
        }
    }

    fn count(&self) -> Option<u16> {
        None
    }

    fn width(&self) -> u8 {
        self.width
    }

    fn corrupts(&self, _host_width: u8) -> bool {
        false
    }

    /// After CMD55 only ACMD6 (R1) and ACMD41 (R3) are sent.
    fn flags(&self, idx: u8) -> Option<u16> {
        match (self.app, idx) {
            (true, 6) => Some(0x1a),
            (true, 41) => Some(0x02),
            (true, _) => None,
            _ => FLAGS.iter().find(|(i, _)| *i == idx).map(|&(_, f)| f),
        }
    }

    fn carries_data(&self, idx: u8) -> bool {
        matches!(idx, 17 | 18 | 24 | 25)
    }
}
