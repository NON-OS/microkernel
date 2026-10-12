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

//! The download in fragments of at most 252 octets, each sent with an index
//! octet in one Download command. The index counts from 0 to 7F and then
//! from 1 again, and bit 7 marks the last fragment. There is always one
//! more fragment than whole 252-octet pieces, so an image that is an exact
//! multiple ends with an empty last fragment: the controller is told it
//! has ended either way.

pub const FRAG_LEN: usize = 252;
pub const LAST: u8 = 0x80;

/// Each fragment's index octet and data, in order.
pub fn fragments(image: &[u8]) -> impl Iterator<Item = (u8, &[u8])> {
    let count = image.len() / FRAG_LEN + 1;
    let mut next = 0u8;
    (0..count).map(move |k| {
        let index = next;
        next = if next == 0x7f { 1 } else { next + 1 };
        let start = k * FRAG_LEN;
        let data = &image[start..(start + FRAG_LEN).min(image.len())];
        (if k + 1 == count { index | LAST } else { index }, data)
    })
}
