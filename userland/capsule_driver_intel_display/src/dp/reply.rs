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

//! The reply command of an AUX transaction (DisplayPort 1.4a section
//! 2.7): native and I2C answers in their own two bits each.

use super::aux::Reply;

/// Bits 5:4 of a native reply's command byte (bits 7:6 for I2C):
/// 00 ACK, 01 NACK, 10 DEFER (section 2.7.7.1.4 and 2.7.7.1.5).
pub fn native_reply(b: u8) -> Option<Reply> {
    decode((b >> 4) & 3)
}

pub fn i2c_reply(b: u8) -> Option<Reply> {
    decode((b >> 6) & 3)
}

fn decode(code: u8) -> Option<Reply> {
    match code {
        0 => Some(Reply::Ack),
        1 => Some(Reply::Nack),
        2 => Some(Reply::Defer),
        _ => None,
    }
}
