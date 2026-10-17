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
//! An address as text: 0x and its 40 hex digits in their EIP-55 spelling,
//! a letter a capital where its nibble of the keccak of the lower-case
//! digits is 8 or more. In full, never shortened: what a payee is shown as
//! wherever a person checks one screen against another.

use super::hashers::Keccak;

pub fn address_text(k: &impl Keccak, a: &[u8; 20]) -> [u8; 42] {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut digits = [0u8; 40];
    for (i, b) in a.iter().enumerate() {
        digits[2 * i] = HEX[usize::from(b >> 4)];
        digits[2 * i + 1] = HEX[usize::from(b & 15)];
    }
    let hash = k.keccak256(&digits);
    let mut out = [0u8; 42];
    out[..2].copy_from_slice(b"0x");
    for (i, c) in digits.iter().enumerate() {
        let nibble = if i % 2 == 0 { hash[i / 2] >> 4 } else { hash[i / 2] & 15 };
        out[2 + i] = if nibble >= 8 { c.to_ascii_uppercase() } else { *c };
    }
    out
}
