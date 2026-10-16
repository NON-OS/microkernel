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

//! The SD card in the slot: its CID and CSD laid out as the SD Physical
//! Layer (5.2, 5.3.3) has them, where it is in the state machine of 4.3,
//! and its blocks, zero until written.

use std::collections::HashMap;

use crate::fields::put;

#[derive(Clone, Default)]
pub struct Card {
    pub cid: [u32; 4],
    pub csd: [u32; 4],
    pub high_capacity: bool,
    /// Answers CMD8: a card of version 2.00 or later.
    pub v2: bool,
    /// The write-protect switch on its side, which only the host reads.
    pub wp: bool,
    /// ACMD41 answers busy this many times before the card is ready.
    pub busy_polls: u32,
    /// CMD13 answers programming this many times after each write.
    pub program_polls: u32,
    /// R1 error bits the next CMD13 reports, once.
    pub errors: u32,
    pub blocks: u64,
    pub data: HashMap<u64, Vec<u8>>,
    pub(super) rca: u16,
    pub(super) state: u32,
    pub(super) app: bool,
    /// Where the next data phase starts, in blocks, and whether it runs on
    /// until CMD12.
    pub(super) at: u64,
    pub(super) multi: bool,
    pub(super) programming: u32,
}

impl Card {
    /// An SDHC card of `blocks` (a multiple of 1024) whose serial is `psn`.
    pub fn sdhc(blocks: u64, psn: u32) -> Card {
        let mut csd = [0u32; 4];
        put(&mut csd, 126, 2, 1);
        put(&mut csd, 48, 22, blocks / 1024 - 1);
        Card { csd, blocks, high_capacity: true, ..Card::named(psn) }
    }

    /// A byte addressed card with 512-byte blocks: (C_SIZE + 1) * 512 of them.
    pub fn sdsc(c_size: u64, psn: u32) -> Card {
        let mut csd = [0u32; 4];
        put(&mut csd, 62, 12, c_size);
        put(&mut csd, 47, 3, 7);
        put(&mut csd, 80, 4, 9);
        Card { csd, blocks: (c_size + 1) << 9, ..Card::named(psn) }
    }
}
