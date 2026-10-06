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

//! Base64 as directory documents use it, padded or not.

extern crate alloc;

use alloc::vec::Vec;

/// Decode base64, with or without padding.
///
pub fn decode(text: &[u8]) -> Option<Vec<u8>> {
    let mut out = Vec::with_capacity(text.len() * 3 / 4 + 3);
    let mut acc = 0u32;
    let mut bits = 0u32;
    for &byte in text {
        if byte.is_ascii_whitespace() {
            continue;
        }
        if byte == b'=' {
            break;
        }
        let value = symbol(byte)?;
        acc = (acc << 6) | value as u32;
        bits += 6;
        if bits >= 8 {
            bits -= 8;
            out.push((acc >> bits) as u8);
        }
    }
    /*
     * Leftover bits are the tail of the last group and must be zero. A
     * non-zero remainder means the text carried bits that decode to nothing,
     * which is how two different strings end up decoding to the same bytes.
     * For an identity fingerprint that is a way to name one relay by two
     * names, so it is refused.
     */
    if acc & ((1u32 << bits) - 1) != 0 {
        return None;
    }
    Some(out)
}

fn symbol(byte: u8) -> Option<u8> {
    match byte {
        b'A'..=b'Z' => Some(byte - b'A'),
        b'a'..=b'z' => Some(byte - b'a' + 26),
        b'0'..=b'9' => Some(byte - b'0' + 52),
        b'+' => Some(62),
        b'/' => Some(63),
        _ => None,
    }
}
