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

//! SET_REPORT: command register, type/id + opcode, then the data register
//! and the length-prefixed report.

use super::command::{Command, REPORT_MAX};
use crate::i2c_client::write_read;

const OPCODE_SET_REPORT: u8 = 0x03;

pub(super) fn set(port: u32, addr: u8, cmd: &Command, report: &[u8]) -> bool {
    let report_len = report.len();
    let len = (2 + report_len) as u16;
    let l = len.to_le_bytes();
    let mut tx = [0u8; 9 + REPORT_MAX];
    tx[..cmd.head_len].copy_from_slice(&cmd.head[..cmd.head_len]);
    tx[3] = OPCODE_SET_REPORT;
    let at = cmd.head_len;
    tx[at..at + 2].copy_from_slice(&cmd.data);
    tx[at + 2..at + 4].copy_from_slice(&l);
    tx[at + 4..at + 4 + report_len].copy_from_slice(report);
    write_read(port, addr, &tx[..at + 4 + report_len], &mut []).is_some()
}
