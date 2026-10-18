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

//! The two answers that change with time: ACMD41 while the card powers up,
//! CMD13 while it programs what was written.

use super::card::Card;
use super::card_cmd::Reply;
use super::linux::{PRG, READY, TRAN};

/// ACMD41's voltage window, 2.7 to 3.6 V, as cards report it.
const OCR_WINDOW: u32 = 0x00FF_8000;
const BUSY_DONE: u32 = 1 << 31;
const CCS: u32 = 1 << 30;

impl Card {
    /// CMD13: programming for a few polls after a write, then back in
    /// transfer state and ready for data.
    pub(super) fn poll(&mut self) -> u32 {
        if self.programming > 0 {
            self.programming -= 1;
        } else if self.state == PRG {
            self.state = TRAN;
        }
        self.status() | std::mem::take(&mut self.errors)
    }

    /// ACMD41: busy for a few polls, then ready, high capacity only when
    /// the host asked for it (HCS) and the card is.
    pub(super) fn op_cond(&mut self, arg: u32) -> Reply {
        if self.busy_polls > 0 {
            self.busy_polls -= 1;
            return Reply::Short(0x3F, OCR_WINDOW);
        }
        self.state = READY;
        let ccs = if self.high_capacity && arg & CCS != 0 { CCS } else { 0 };
        Reply::Short(0x3F, BUSY_DONE | ccs | OCR_WINDOW)
    }
}
