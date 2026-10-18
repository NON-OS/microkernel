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

//! The five SMBus protocols the host runs, and their command bits.

use crate::regs::{CNT_BLOCK_DATA, CNT_BYTE, CNT_BYTE_DATA, CNT_QUICK, CNT_WORD_DATA};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Kind {
    Quick,
    /// Send or receive one byte; a send carries it in the command.
    Byte,
    ByteData,
    WordData,
    Block,
}

impl Kind {
    pub(super) fn from_wire(k: u8) -> Option<Kind> {
        Some(
            [Kind::Quick, Kind::Byte, Kind::ByteData, Kind::WordData, Kind::Block]
                .get(k as usize)?,
        )
        .copied()
    }

    pub fn control(self) -> u8 {
        match self {
            Kind::Quick => CNT_QUICK,
            Kind::Byte => CNT_BYTE,
            Kind::ByteData => CNT_BYTE_DATA,
            Kind::WordData => CNT_WORD_DATA,
            Kind::Block => CNT_BLOCK_DATA,
        }
    }
}
