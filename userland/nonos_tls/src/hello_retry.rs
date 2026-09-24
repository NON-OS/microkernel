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

//! Recognising a HelloRetryRequest, which is a ServerHello wearing a disguise.

/*
 * RFC 8446 section 4.1.3. A retry is a ServerHello whose random is this fixed
 * value, so a client that does not compare it reads a retry as a hello and
 * waits for a message that has already arrived.
 */
pub const HELLO_RETRY_RANDOM: [u8; 32] = [
    0xCF, 0x21, 0xAD, 0x74, 0xE5, 0x9A, 0x61, 0x11, 0xBE, 0x1D, 0x8C, 0x02, 0x1E, 0x65, 0xB8, 0x91,
    0xC2, 0xA2, 0x11, 0x16, 0x7A, 0xBB, 0x8C, 0x5E, 0x07, 0x9E, 0x09, 0xE2, 0xC8, 0xA8, 0x33, 0x9C,
];

/// Whether this handshake message is a HelloRetryRequest.
///
pub fn is_hello_retry(handshake: &[u8]) -> bool {
    if handshake.first() != Some(&2) {
        return false;
    }
    // Type, three length bytes, two version bytes, then the thirty two random.
    match handshake.get(6..38) {
        Some(random) => random == HELLO_RETRY_RANDOM,
        None => false,
    }
}

/// Whether the first record buffered is a HelloRetryRequest.
///
pub fn in_buffer(buf: &[u8]) -> bool {
    if buf.len() < 5 || buf[0] != super::constants::TLS_HANDSHAKE {
        return false;
    }
    let len = u16::from_be_bytes([buf[3], buf[4]]) as usize;
    match buf.get(5..5 + len) {
        Some(msg) => is_hello_retry(msg),
        None => false,
    }
}
