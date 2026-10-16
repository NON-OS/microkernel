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

//! What makes one card another: its CID (SD Physical Layer 5.2); and the
//! state the card keeps between commands.

use super::card::Card;
use super::card_cmd::Reply;
use super::linux::{IDLE, PRG, READY_FOR_DATA};
use crate::fields::put;

impl Card {
    /// Manufacturer 03h, OEM "SD", product "SC32G", revision 8.0, serial
    /// `psn`, made October 2022.
    pub(super) fn named(psn: u32) -> Card {
        let mut cid = [0u32; 4];
        let fields = [(120, 8, 0x03), (104, 16, 0x5344), (64, 40, 0x53_4333_3247), (56, 8, 0x80)];
        fields.iter().for_each(|&(at, size, v)| put(&mut cid, at, size, v));
        put(&mut cid, 24, 32, psn as u64);
        put(&mut cid, 8, 12, 0x16A);
        Card { cid, v2: true, busy_polls: 2, program_polls: 2, ..Card::default() }
    }

    /// Power gone, or CMD0: idle, with no address.
    pub(super) fn reset(&mut self) {
        (self.state, self.rca, self.app, self.programming) = (IDLE, 0, false, 0);
    }

    /// R1 card status: the current state, and ready for data unless busy.
    pub(super) fn status(&self) -> u32 {
        let ready = if self.state == PRG { 0 } else { READY_FOR_DATA };
        (self.state << 9) | ready
    }

    pub(super) fn then(&mut self, state: u32, reply: Reply) -> Reply {
        self.state = state;
        reply
    }
}
