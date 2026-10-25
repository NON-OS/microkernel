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

//! The Command register word.

use super::super::regs::{CMD_DATA, CMD_TYPE_ABORT};
use super::Resp;

/// The Command register: index in 13:8, type in 7:6, Data Present in 5,
/// the checks and response type below. `abort` marks CMD12 sent to end a
/// transfer (Command Type 11b).
pub const fn command_word(index: u8, resp: Resp, data: bool, abort: bool) -> u16 {
    let mut w = ((index as u16 & 0x3f) << 8) | resp.flags();
    if data {
        w |= CMD_DATA;
    }
    if abort {
        w |= CMD_TYPE_ABORT;
    }
    w
}
