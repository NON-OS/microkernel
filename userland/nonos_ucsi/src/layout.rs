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

//! The mailbox (UCSI 1.2, the data structures chapter; recalled,
//! unverified): VERSION (u16) at 0, CCI (u32) at 4, CONTROL (u64) at 8,
//! MESSAGE IN at 0x10, MESSAGE OUT after it. MESSAGE IN is 16 bytes in
//! revision 1.x and larger in 2.x, at the same offset; every reply read
//! here fits in its first 16 bytes.

pub const VERSION: u64 = 0;
pub const CCI: u64 = 4;
pub const CONTROL: u64 = 8;
pub const MESSAGE_IN: u64 = 0x10;
pub const MESSAGE_IN_READ: usize = 16;
/// The smallest mailbox a revision 1.x PPM shares: through MESSAGE OUT.
pub const MAILBOX_MIN: u64 = 0x30;

/// Command Status and Connector Change Indication.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Cci {
    /// The connector whose state changed, 0 for none (bits 7:1).
    pub connector: u8,
    /// Bytes of MESSAGE IN the completed command filled (bits 15:8).
    pub data_len: u8,
    pub not_supported: bool,
    pub cancel_completed: bool,
    pub reset_completed: bool,
    pub busy: bool,
    pub ack_command: bool,
    pub error: bool,
    pub command_completed: bool,
}

impl Cci {
    pub fn decode(v: u32) -> Self {
        let bit = |n: u32| v & (1 << n) != 0;
        Cci {
            connector: ((v >> 1) & 0x7F) as u8,
            data_len: (v >> 8) as u8,
            not_supported: bit(25),
            cancel_completed: bit(26),
            reset_completed: bit(27),
            busy: bit(28),
            ack_command: bit(29),
            error: bit(30),
            command_completed: bit(31),
        }
    }
}
