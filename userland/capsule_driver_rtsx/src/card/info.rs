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

//! What the identification found out about the card.

use crate::sd::Cid;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Card {
    /// The relative card address from CMD3.
    pub rca: u16,
    /// Block addressed (SDHC, SDXC) rather than byte addressed.
    pub high_capacity: bool,
    /// Size in 512-byte blocks, from the CSD, all of it addressable.
    pub blocks: u64,
    /// Which card this is; Debug prints none of it.
    pub cid: Cid,
}

impl Card {
    /// The same card as `other`, at whatever address it answers now: its
    /// CID, and the size and addressing that go with it.
    pub fn same(&self, other: &Card) -> bool {
        self.cid == other.cid
            && (self.blocks, self.high_capacity) == (other.blocks, other.high_capacity)
    }
}
