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

//! The commands one transfer is made of, by its size and the card.

use super::super::cmds::{
    READ_MULTIPLE_BLOCK, READ_SINGLE_BLOCK, WRITE_BLOCK, WRITE_MULTIPLE_BLOCK,
};

/// The commands one transfer is made of.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Plan {
    /// SET_BLOCK_COUNT goes first.
    pub cmd23: bool,
    /// The data command.
    pub index: u8,
    pub multi: bool,
    /// The host sends CMD12 after the data.
    pub auto12: bool,
}

/// How `blocks` blocks are read or written on a card that does, or does
/// not, take CMD23. Pure, so the host proofs hold it.
pub const fn plan(blocks: u16, write: bool, card_cmd23: bool) -> Plan {
    if blocks <= 1 {
        let index = if write { WRITE_BLOCK } else { READ_SINGLE_BLOCK };
        return Plan { cmd23: false, index, multi: false, auto12: false };
    }
    let index = if write { WRITE_MULTIPLE_BLOCK } else { READ_MULTIPLE_BLOCK };
    Plan { cmd23: card_cmd23, index, multi: true, auto12: !card_cmd23 }
}
