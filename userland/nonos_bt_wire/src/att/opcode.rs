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

//! ATT opcodes (Vol 3, Part F, 3.4.8). Bit 6 marks a command, which gets
//! no response; a server ignores a command it does not support but must
//! answer every request (Vol 3, Part F, 3.3).

pub const ERROR_RSP: u8 = 0x01;
pub const EXCHANGE_MTU_REQ: u8 = 0x02;
pub const EXCHANGE_MTU_RSP: u8 = 0x03;
pub const FIND_INFORMATION_REQ: u8 = 0x04;
pub const FIND_INFORMATION_RSP: u8 = 0x05;
pub const FIND_BY_TYPE_VALUE_REQ: u8 = 0x06;
pub const FIND_BY_TYPE_VALUE_RSP: u8 = 0x07;
pub const READ_BY_TYPE_REQ: u8 = 0x08;
pub const READ_BY_TYPE_RSP: u8 = 0x09;
pub const READ_REQ: u8 = 0x0a;
pub const READ_RSP: u8 = 0x0b;
pub const READ_BY_GROUP_TYPE_REQ: u8 = 0x10;
pub const READ_BY_GROUP_TYPE_RSP: u8 = 0x11;
pub const WRITE_REQ: u8 = 0x12;
pub const WRITE_RSP: u8 = 0x13;
pub const HANDLE_VALUE_NTF: u8 = 0x1b;
pub const HANDLE_VALUE_IND: u8 = 0x1d;
pub const HANDLE_VALUE_CFM: u8 = 0x1e;
pub const WRITE_CMD: u8 = 0x52;

/// The command flag (Vol 3, Part F, 3.3.1).
pub const COMMAND_FLAG: u8 = 0x40;

/// Whether `opcode` is a request a server must answer: neither a command
/// nor one of the responses, notifications, indications or confirmations.
pub fn is_request(opcode: u8) -> bool {
    let answers = [ERROR_RSP, EXCHANGE_MTU_RSP, FIND_INFORMATION_RSP, FIND_BY_TYPE_VALUE_RSP];
    let more = [READ_BY_TYPE_RSP, READ_RSP, READ_BY_GROUP_TYPE_RSP, WRITE_RSP];
    let pushes = [HANDLE_VALUE_NTF, HANDLE_VALUE_IND, HANDLE_VALUE_CFM];
    opcode & COMMAND_FLAG == 0
        && !answers.contains(&opcode)
        && !more.contains(&opcode)
        && !pushes.contains(&opcode)
}
