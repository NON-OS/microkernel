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

//! The capabilities string's messages (DDC/CI, as this driver reads the
//! standard; unverified against the document). A request names the offset
//! it wants, high byte first; the display answers with the same offset and
//! up to 32 bytes of the string after it, and with none past its end. The
//! reply is framed as a VCP reply is: the display's address, a length byte
//! (0x80 and the count), the message and a checksum seeded with REPLY_SEED.

use super::error::DdcError;
use super::frame::{xor, DISPLAY, HOST, REPLY_SEED};

pub const CAPS_REQUEST: u8 = 0xF3;
pub const CAPS_REPLY: u8 = 0xE3;
/// The most string one reply carries.
pub const FRAGMENT_MAX: usize = 32;
/// Read whole whatever the reply's length: address, length byte, opcode,
/// offset, the most string, checksum. Bytes past the length are not read.
pub const CAPS_READ: usize = 5 + FRAGMENT_MAX + 1;

/// Capabilities Request for the string from `offset`.
pub fn caps_request(offset: u16) -> [u8; 6] {
    let [hi, lo] = offset.to_be_bytes();
    let mut r = [HOST, 0x83, CAPS_REQUEST, hi, lo, 0];
    r[5] = xor(DISPLAY, &r[..5]);
    r
}

/// The string bytes a reply to `offset` carries, empty at the end. Every
/// framing byte is checked first: address, length, checksum, opcode, the
/// offset echoed.
pub fn fragment(offset: u16, r: &[u8; CAPS_READ]) -> Result<&[u8], DdcError> {
    if r[0] != DISPLAY {
        return Err(DdcError::Reply);
    }
    let n = (r[1] & 0x7F) as usize;
    if r[1] & 0x80 == 0 || !(3..=3 + FRAGMENT_MAX).contains(&n) {
        return Err(DdcError::Length);
    }
    if xor(REPLY_SEED, &r[..3 + n]) != 0 {
        return Err(DdcError::Checksum);
    }
    if r[2] != CAPS_REPLY || u16::from_be_bytes([r[3], r[4]]) != offset {
        return Err(DdcError::Reply);
    }
    Ok(&r[5..2 + n])
}
