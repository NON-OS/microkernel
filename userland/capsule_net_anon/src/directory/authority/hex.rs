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

//! Turning a forty character fingerprint literal into twenty bytes at compile
//! time, so a typo is a build failure and not an unreachable authority.

pub(super) const fn hex(text: &[u8; 40]) -> [u8; 20] {
    let mut out = [0u8; 20];
    let mut index = 0;
    while index < 20 {
        out[index] = (nibble(text[index * 2]) << 4) | nibble(text[index * 2 + 1]);
        index += 1;
    }
    out
}

/*
 * Every byte that is not a hex digit maps to an index one past the end of a one
 * element table, so it is a compile error naming this line rather than a wrong
 * fingerprint. The check has to be the indexing itself: a const fn cannot
 * return an error, and a sentinel value would decode to a plausible byte.
 */
const fn nibble(byte: u8) -> u8 {
    let value = VALUE[byte as usize];
    let refuse = [0u8; 1];
    // Zero for a digit, one for anything else, and one is out of range.
    refuse[(value >> 7) as usize];
    value & 0x0f
}

/// The value of each hex digit in the low nibble, with the high bit set on every
/// byte that is not one.
const VALUE: [u8; 256] = build();

const fn build() -> [u8; 256] {
    let mut table = [0x80u8; 256];
    let mut i = 0;
    while i < 10 {
        table[b'0' as usize + i] = i as u8;
        i += 1;
    }
    let mut i = 0;
    while i < 6 {
        table[b'A' as usize + i] = 10 + i as u8;
        table[b'a' as usize + i] = 10 + i as u8;
        i += 1;
    }
    table
}
