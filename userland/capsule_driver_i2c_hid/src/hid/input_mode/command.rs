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

//! The GET_REPORT command for one feature report, built from the HID
//! descriptor's command and data registers. SET_REPORT reuses it with the
//! opcode swapped.

/// The longest feature report handled, report id byte included.
pub(super) const REPORT_MAX: usize = 62;

const OPCODE_GET_REPORT: u8 = 0x02;
const REPORT_TYPE_FEATURE: u8 = 3;

pub(super) struct Command {
    /// Command register, type and id, opcode, and the id again when it does
    /// not fit the nibble.
    pub head: [u8; 5],
    pub head_len: usize,
    /// The data register, little-endian.
    pub data: [u8; 2],
}

/// None when the descriptor names no command or data register.
pub(super) fn command(desc: &[u8; 30], id: u8) -> Option<Command> {
    let cmd = u16::from_le_bytes([desc[16], desc[17]]);
    let data = u16::from_le_bytes([desc[18], desc[19]]);
    if cmd == 0 || data == 0 {
        return None;
    }
    let c = cmd.to_le_bytes();
    // Report ids of 15 and up do not fit the id nibble: the nibble reads 0xF
    // and the id follows the opcode as a third byte (HID over I2C v1.0
    // section 7.2, Linux i2c_hid_encode_command).
    let (ty_id, ext) = if id >= 0x0F {
        ((REPORT_TYPE_FEATURE << 4) | 0x0F, Some(id))
    } else {
        ((REPORT_TYPE_FEATURE << 4) | id, None)
    };
    let mut head = [0u8; 5];
    head[..4].copy_from_slice(&[c[0], c[1], ty_id, OPCODE_GET_REPORT]);
    let head_len = if let Some(id) = ext {
        head[4] = id;
        5
    } else {
        4
    };
    Some(Command { head, head_len, data: data.to_le_bytes() })
}
