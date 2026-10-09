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

use alloc::vec::Vec;

use super::composite::composite;
use super::cursor::Cursor;
use super::glyph::simple;
use super::streams::streams;

/// The rebuilt glyf and loca tables, and each glyph's xMin for hmtx.
pub(super) struct Outlines {
    pub(super) glyf: Vec<u8>,
    pub(super) loca: Vec<u8>,
    pub(super) x_mins: Vec<i16>,
}

/// Rebuild glyf and loca from the transformed glyf table. `loca_len` is
/// the sfnt length the directory gives loca; `long_loca` the head's
/// indexToLocFormat, which the transform must repeat.
pub(super) fn rebuild(t: &[u8], loca_len: usize, long_loca: bool) -> Option<Outlines> {
    let mut c = Cursor::new(t);
    let (_version, options, n, format) = (c.u16()?, c.u16()?, c.u16()? as usize, c.u16()?);
    if format > 1 || (format == 1) != long_loca || loca_len != (n + 1) * (2 + 2 * format as usize) {
        return None;
    }
    let mut s = streams(&mut c)?;
    let overlap = if options & 1 != 0 { Some(c.bytes(n.div_ceil(8))?) } else { None };
    let boxed = s.bboxes.bytes(n.div_ceil(32) * 4)?;
    let mut out = Outlines { glyf: Vec::new(), loca: Vec::new(), x_mins: Vec::new() };
    out.x_mins.try_reserve_exact(n).ok()?;
    let mut offsets: Vec<usize> = Vec::new();
    offsets.try_reserve_exact(n + 1).ok()?;
    for i in 0..n {
        offsets.push(out.glyf.len());
        let bit = |map: &[u8]| map[i >> 3] & (0x80 >> (i & 7)) != 0;
        let bbox = if bit(boxed) { Some(s.bboxes.bytes(8)?) } else { None };
        let x_min = match s.contours.u16()? as i16 {
            0 if bbox.is_some() => return None,
            0 => 0,
            -1 => composite(&mut s, bbox?, &mut out.glyf)?,
            k if k > 0 => {
                simple(&mut s, k as usize, bbox, overlap.is_some_and(bit), &mut out.glyf)?
            }
            _ => return None,
        };
        out.x_mins.push(x_min);
        out.glyf.resize(out.glyf.len().next_multiple_of(4), 0);
    }
    offsets.push(out.glyf.len());
    for off in offsets {
        match format {
            0 => out.loca.extend_from_slice(&u16::try_from(off / 2).ok()?.to_be_bytes()),
            _ => out.loca.extend_from_slice(&u32::try_from(off).ok()?.to_be_bytes()),
        }
    }
    Some(out)
}
