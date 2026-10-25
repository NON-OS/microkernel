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

//! Report Maps for the proofs, written item by item from HID 1.11, 6.2.2
//! and the usage pages of HUT 1.4.

/// A keyboard as report 1: eight modifier bits, a constant octet, six key
/// slots as an array, and five LED bits out with three of padding; then
/// a Consumer Control array as report 2. 90 octets, so a Read and four
/// Read Blobs.
#[rustfmt::skip]
pub const KEYBOARD: [u8; 90] = [
    0x05, 0x01, 0x09, 0x06, 0xa1, 0x01, 0x85, 0x01,
    0x05, 0x07, 0x19, 0xe0, 0x29, 0xe7, 0x15, 0x00, 0x25, 0x01,
    0x75, 0x01, 0x95, 0x08, 0x81, 0x02,
    0x95, 0x01, 0x75, 0x08, 0x81, 0x01,
    0x95, 0x05, 0x75, 0x01, 0x05, 0x08, 0x19, 0x01, 0x29, 0x05, 0x91, 0x02,
    0x95, 0x01, 0x75, 0x03, 0x91, 0x01,
    0x95, 0x06, 0x75, 0x08, 0x15, 0x00, 0x25, 0x65,
    0x05, 0x07, 0x19, 0x00, 0x29, 0x65, 0x81, 0x00,
    0xc0,
    0x05, 0x0c, 0x09, 0x01, 0xa1, 0x01, 0x85, 0x02,
    0x15, 0x00, 0x26, 0xff, 0x03, 0x19, 0x00, 0x2a, 0xff, 0x03,
    0x75, 0x10, 0x95, 0x01, 0x81, 0x00,
    0xc0,
];

/// The same keyboard without Report IDs and without the consumer report.
pub fn unnumbered() -> Vec<u8> {
    let mut m = KEYBOARD[..65].to_vec();
    m.drain(6..8);
    m
}

/// An item: prefix from type and tag, data in the fewest octets given.
pub fn item(kind: u8, tag: u8, data: &[u8]) -> Vec<u8> {
    let size = [0u8, 1, 2, 3, 3][data.len()];
    [&[tag << 4 | kind << 2 | size][..], data].concat()
}

pub fn main(tag: u8, data: u8) -> Vec<u8> {
    item(0, tag, &[data])
}

pub fn global(tag: u8, data: u8) -> Vec<u8> {
    item(1, tag, &[data])
}

pub fn local(tag: u8, data: u8) -> Vec<u8> {
    item(2, tag, &[data])
}

/// One input field of `count` one-bit usages from `min`, on the Keyboard
/// page, as report `id` (0 for none).
pub fn bits_field(id: u8, min: u8, count: u8) -> Vec<u8> {
    let id = if id == 0 { vec![] } else { global(8, id) };
    let max = min + count - 1;
    let items = [global(0, 7), local(1, min), local(2, max), global(7, 1), global(9, count)];
    [id, items.concat(), global(2, 1), main(8, 0x02)].concat()
}
