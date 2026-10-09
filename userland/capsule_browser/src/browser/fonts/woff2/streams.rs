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

use super::cursor::Cursor;

/// The seven streams of a transformed glyf table (WOFF2 5.1).
pub(super) struct Streams<'a> {
    pub(super) contours: Cursor<'a>,
    pub(super) points: Cursor<'a>,
    pub(super) flags: Cursor<'a>,
    pub(super) glyphs: Cursor<'a>,
    pub(super) composites: Cursor<'a>,
    pub(super) bboxes: Cursor<'a>,
    pub(super) code: Cursor<'a>,
}

/// The seven stream lengths, then the streams back to back.
pub(super) fn streams<'a>(c: &mut Cursor<'a>) -> Option<Streams<'a>> {
    let mut sizes = [0usize; 7];
    for size in &mut sizes {
        *size = c.u32()? as usize;
    }
    let mut take = |i: usize| c.bytes(sizes[i]).map(Cursor::new);
    Some(Streams {
        contours: take(0)?,
        points: take(1)?,
        flags: take(2)?,
        glyphs: take(3)?,
        composites: take(4)?,
        bboxes: take(5)?,
        code: take(6)?,
    })
}
