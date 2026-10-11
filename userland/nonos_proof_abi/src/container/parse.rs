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

//! NPRF v1: `"NPRF" | version u16 | kind u16 | flags u16 | n u16`, then n
//! sections `tag u16 | len u32 | bytes`, tags strictly ascending, all of it
//! at most 1 MiB, nothing after the last section.

use super::kind::Kind;
use super::tag::{fits, LAST};
use crate::words::{u16_at, u32_at};

pub const MAGIC: &[u8; 4] = b"NPRF";
pub const VERSION: u16 = 1;
pub const MAX_BYTES: usize = 1 << 20;
pub(super) const HEAD: usize = 12;
pub(super) const SECTION_HEAD: usize = 6;

pub struct Container<'a> {
    pub kind: Kind,
    sections: [Option<&'a [u8]>; LAST as usize + 1],
}

impl<'a> Container<'a> {
    pub fn get(&self, tag: u16) -> Option<&'a [u8]> {
        *self.sections.get(usize::from(tag))?
    }
}

/// `None` for anything off the format: an unknown version, kind, flag or tag,
/// a section of the wrong length, tags out of order or repeated, a count that
/// disagrees with the bytes, or a trailing byte.
pub fn parse(b: &[u8]) -> Option<Container<'_>> {
    if b.len() > MAX_BYTES || b.get(..4)? != MAGIC || u16_at(b, 4)? != VERSION {
        return None;
    }
    let kind = Kind::from_u16(u16_at(b, 6)?)?;
    if u16_at(b, 8)? != 0 {
        return None;
    }
    let n = u16_at(b, 10)?;
    let mut sections = [None; LAST as usize + 1];
    let (mut at, mut last) = (HEAD, 0u16);
    for _ in 0..n {
        let tag = u16_at(b, at)?;
        let len = u32_at(b, at + 2)? as usize;
        if tag <= last || tag > LAST || !fits(tag, len) {
            return None;
        }
        let start = at + SECTION_HEAD;
        let end = start.checked_add(len)?;
        sections[usize::from(tag)] = Some(b.get(start..end)?);
        (at, last) = (end, tag);
    }
    (at == b.len()).then_some(Container { kind, sections })
}
