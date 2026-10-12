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

//! A section's body in the second layout: a two-octet subsection count and
//! two reserved octets, then the subsections, each an eco (1 octet), a
//! priority (1), a key id in security sections (1), one octet more, a
//! length (4) and its data. A subsection is for the ROM whose version plus
//! one is its eco.

use alloc::vec::Vec;

use super::refused::Layout;
use super::rtbt::le32;

pub struct Pick<'a> {
    pub prio: u8,
    pub data: &'a [u8],
}

/// Each subsection of `body` for this ROM, and for `key` when the section
/// is keyed, put into `picked` ahead of the first of no lower priority.
/// Equal priorities thus run later subsections first, the order the
/// controllers were brought up with in Linux's driver.
pub fn subsections<'a>(
    body: &'a [u8],
    rom: u8,
    key: Option<u8>,
    picked: &mut Vec<Pick<'a>>,
) -> Result<(), Layout> {
    let count = body.get(..4).map(|h| u16::from_le_bytes([h[0], h[1]])).ok_or(Layout::Bounds)?;
    let mut at = 4;
    for _ in 0..count {
        let head = body.get(at..at + 8).ok_or(Layout::Bounds)?;
        let len = le32(head, 4) as usize;
        let data = body.get(at + 8..).and_then(|r| r.get(..len)).ok_or(Layout::Bounds)?;
        at += 8 + len;
        let (eco, prio, id) = (head[0], head[1], head[2]);
        if u16::from(eco) != u16::from(rom) + 1 || key.is_some_and(|k| k != id) {
            continue;
        }
        let place = picked.iter().position(|p| p.prio >= prio).unwrap_or(picked.len());
        picked.insert(place, Pick { prio, data });
    }
    Ok(())
}
