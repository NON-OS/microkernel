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

/// Bitcoin-style base58, which is how Nym renders every key in an address.
const ALPHABET: &[u8; 58] = b"123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";

/// Decode into exactly 32 bytes, accepting only the canonical encoding.
///
/// In base58 each leading '1' stands for one leading zero byte and the rest
/// is the number, so a 32-byte key has exactly one spelling. The decoder used
/// to skip that count: empty text decoded to the all-zero key, and "1A",
/// "11A" and "A" decoded to the same one. A key that arrives with a spelling
/// its owner never published is refused rather than read as some other key.
pub fn decode32(text: &[u8]) -> Option<[u8; 32]> {
    if text.is_empty() {
        return None;
    }
    let ones = text.iter().take_while(|&&c| c == b'1').count();
    let mut acc = [0u8; 32];
    for ch in &text[ones..] {
        let digit = ALPHABET.iter().position(|a| a == ch)? as u32;
        let mut carry = digit;
        for byte in acc.iter_mut().rev() {
            let value = (*byte as u32) * 58 + carry;
            *byte = value as u8;
            carry = value >> 8;
        }
        if carry != 0 {
            return None;
        }
    }
    /* The number's own leading zero bytes must be exactly the ones spelt. */
    let zeros = acc.iter().take_while(|&&b| b == 0).count();
    if zeros != ones {
        return None;
    }
    Some(acc)
}
