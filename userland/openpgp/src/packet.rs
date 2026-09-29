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

//! Packet framing, old and new format. Partial and indeterminate lengths are
//! for streamed literal data, never for keys or signatures, and are refused.

pub struct Packet<'a> {
    pub tag: u8,
    pub body: &'a [u8],
}

fn be(d: &[u8]) -> usize {
    d.iter().fold(0, |v, &b| v << 8 | usize::from(b))
}

/// The first packet of `d`, and what follows it.
pub fn next(d: &[u8]) -> Option<(Packet<'_>, &[u8])> {
    let b = *d.first()?;
    if b & 0x80 == 0 {
        return None;
    }
    let (tag, len, head) = if b & 0x40 != 0 {
        let (len, head) = match usize::from(*d.get(1)?) {
            o @ 0..=191 => (o, 2),
            o @ 192..=223 => (((o - 192) << 8) + usize::from(*d.get(2)?) + 192, 3),
            255 => (be(d.get(2..6)?), 6),
            _ => return None,
        };
        (b & 0x3F, len, head)
    } else {
        let head = match b & 3 {
            0 => 2,
            1 => 3,
            2 => 5,
            _ => return None,
        };
        ((b >> 2) & 0x0F, be(d.get(1..head)?), head)
    };
    let end = head.checked_add(len)?;
    Some((Packet { tag, body: d.get(head..end)? }, &d[end..]))
}
