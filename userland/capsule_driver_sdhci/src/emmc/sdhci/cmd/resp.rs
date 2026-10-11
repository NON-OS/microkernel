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

//! The response types a command expects and their Command register flags.

use super::super::regs::{
    CMD_CRC_CHECK, CMD_INDEX_CHECK, CMD_RESP_136, CMD_RESP_48, CMD_RESP_48_BUSY, CMD_RESP_NONE,
};

/// The response a command expects (JEDEC eMMC 5.1, 6.12).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Resp {
    /// No response (CMD0).
    None,
    /// 48 bits, CRC and index checked: card status.
    R1,
    /// R1 with busy signalled on DAT0 after it.
    R1b,
    /// 136 bits, CRC checked, no index: CID or CSD.
    R2,
    /// 48 bits, neither CRC nor index: OCR.
    R3,
}

impl Resp {
    /// The command waits on DAT0 busy, so it holds the DAT line.
    pub const fn busy(self) -> bool {
        matches!(self, Resp::R1b)
    }

    pub const fn long(self) -> bool {
        matches!(self, Resp::R2)
    }

    /// Response Type Select, CRC Check Enable and Index Check Enable.
    pub const fn flags(self) -> u16 {
        match self {
            Resp::None => CMD_RESP_NONE,
            Resp::R1 => CMD_RESP_48 | CMD_CRC_CHECK | CMD_INDEX_CHECK,
            Resp::R1b => CMD_RESP_48_BUSY | CMD_CRC_CHECK | CMD_INDEX_CHECK,
            Resp::R2 => CMD_RESP_136 | CMD_CRC_CHECK,
            Resp::R3 => CMD_RESP_48,
        }
    }
}
