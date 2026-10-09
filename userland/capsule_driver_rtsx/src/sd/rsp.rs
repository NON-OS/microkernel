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

//! The response a command expects, and the SD_CFG2 type the reader is told
//! for it.

use crate::regs::sd::{
    SD_RSP_TYPE_R0, SD_RSP_TYPE_R1, SD_RSP_TYPE_R1B, SD_RSP_TYPE_R2, SD_RSP_TYPE_R3,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rsp {
    None,
    /// R1, and R6 and R7, which share its length and CRC.
    R1,
    R1b,
    R2,
    /// R3 carries no CRC.
    R3,
}

impl Rsp {
    /// The SD_CFG2 response type, as sd_response_type maps it.
    pub const fn cfg2(self) -> u8 {
        match self {
            Rsp::None => SD_RSP_TYPE_R0,
            Rsp::R1 => SD_RSP_TYPE_R1,
            Rsp::R1b => SD_RSP_TYPE_R1B,
            Rsp::R2 => SD_RSP_TYPE_R2,
            Rsp::R3 => SD_RSP_TYPE_R3,
        }
    }
}
