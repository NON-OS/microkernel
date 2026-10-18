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

//! A resource template's descriptors one by one (ACPI 6.5, 6.4): the small
//! ones (6.4.2) with their 3-bit length, the large ones (6.4.3) with their
//! 16-bit length. The walk stops at the end tag or at a descriptor that
//! runs past the buffer; what came before stands.

use alloc::vec::Vec;

const SMALL_END: u8 = 0x0F;

#[derive(Clone, Copy, Debug)]
pub struct Item<'a> {
    pub large: bool,
    /// The small item's name (bits 6:3) or the large item's (bits 6:0).
    pub kind: u8,
    /// The whole descriptor, header included: GPIO offsets count from it.
    pub whole: &'a [u8],
    pub body: &'a [u8],
}

pub fn items(t: &[u8]) -> Vec<Item<'_>> {
    let (mut out, mut i) = (Vec::new(), 0usize);
    while let Some(&tag) = t.get(i) {
        let large = tag & 0x80 != 0;
        let (kind, head, len) = if large {
            let Some(l) = t.get(i + 1..i + 3) else { break };
            (tag & 0x7F, 3, u16::from_le_bytes([l[0], l[1]]) as usize)
        } else {
            ((tag >> 3) & 0x0F, 1, (tag & 0x07) as usize)
        };
        if !large && kind == SMALL_END {
            break;
        }
        let Some(whole) = t.get(i..i + head + len) else { break };
        out.push(Item { large, kind, whole, body: &whole[head..] });
        i += head + len;
    }
    out
}
