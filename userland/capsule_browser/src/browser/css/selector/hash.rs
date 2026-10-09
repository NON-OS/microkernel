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

/* FNV-1a over a name, salted by its kind (b't' tag, b'#' id, b'.' class) so
 * a class and an id spelled alike hash apart. The selector parser and the
 * per-document sibling table both hash through here, so equal names always
 * meet on equal keys. */
pub fn name_hash(kind: u8, name: &[u8]) -> u64 {
    let mut h: u64 = 0xcbf2_9ce4_8422_2325 ^ kind as u64;
    for &b in name {
        h ^= b as u64;
        h = h.wrapping_mul(0x0000_0100_0000_01b3);
    }
    h
}

/* The key an id is compared by: never zero, so zero can mean "no id". */
pub fn id_key(name: &[u8]) -> u64 {
    name_hash(b'#', name) | 1
}

/* The two bits a key sets in a 128-bit ancestor filter. They come from the
 * top of the hash, where FNV has mixed in every byte of the name. */
pub fn bloom_bits(key: u64) -> [u64; 2] {
    let mut out = [0u64; 2];
    for bit in [key >> 57, (key >> 50) & 127] {
        out[(bit / 64) as usize] |= 1u64 << (bit % 64);
    }
    out
}
