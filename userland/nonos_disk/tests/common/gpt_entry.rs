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

//! GPT entries read out of an array, the type in the canonical text a
//! partition tool prints and the name decoded from UTF-16.

use super::gpt_check::le;

pub struct Entry {
    pub type_text: String,
    pub unique: [u8; 16],
    pub first: u64,
    pub last: u64,
    pub attributes: u64,
    pub name: String,
}

/// The canonical text of on-disk GUID bytes: three little-endian fields
/// reversed, two big-endian ones straight.
fn guid_text(b: &[u8]) -> String {
    let order = [3, 2, 1, 0, 5, 4, 7, 6, 8, 9, 10, 11, 12, 13, 14, 15];
    let hex: Vec<String> = order.iter().map(|&i| format!("{:02X}", b[i])).collect();
    let group = |r: std::ops::Range<usize>| hex[r].concat();
    format!("{}-{}-{}-{}-{}", group(0..4), group(4..6), group(6..8), group(8..10), group(10..16))
}

/// Every entry that is in use, in array order; the rest must be zero.
pub fn entries(array: &[u8]) -> Vec<Entry> {
    let used: Vec<&[u8]> = array.chunks(128).take_while(|e| e[..16] != [0; 16]).collect();
    assert!(array[used.len() * 128..].iter().all(|&b| b == 0), "unused entries are zero");
    let name = |e: &[u8]| {
        let units = e[56..128].chunks(2).map(|c| u16::from_le_bytes([c[0], c[1]]));
        char::decode_utf16(units.take_while(|&u| u != 0)).map(|c| c.unwrap()).collect()
    };
    let entry = |e: &[u8]| Entry {
        type_text: guid_text(&e[0..16]),
        unique: e[16..32].try_into().unwrap(),
        first: le(e, 32, 8),
        last: le(e, 40, 8),
        attributes: le(e, 48, 8),
        name: name(e),
    };
    used.into_iter().map(entry).collect()
}
