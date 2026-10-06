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

use super::points::decode;
use super::store::store_points;
use super::streams::Streams;

/// Append a simple glyph of `contours` contours (WOFF2 5.1 steps 1-7) and
/// return its xMin: the bounding box is the stored one or else the
/// points' own, and instructions follow the contour ends.
pub(super) fn simple(
    s: &mut Streams,
    contours: usize,
    bbox: Option<&[u8]>,
    overlap: bool,
    out: &mut Vec<u8>,
) -> Option<i16> {
    let mut ends = Vec::new();
    ends.try_reserve_exact(contours).ok()?;
    let mut total = 0usize;
    for _ in 0..contours {
        total += s.points.u255()? as usize;
        ends.push(u16::try_from(total.checked_sub(1)?).ok()?);
    }
    let pts = decode(s.flags.bytes(total)?, &mut s.glyphs)?;
    let code_len = s.glyphs.u255()?;
    let code = s.code.bytes(code_len as usize)?;
    let x_min = match bbox {
        Some(b) => {
            out.extend_from_slice(&(contours as i16).to_be_bytes());
            out.extend_from_slice(b);
            i16::from_be_bytes([b[0], b[1]])
        }
        None => {
            let (mut lo, mut hi) = ((pts[0].x, pts[0].y), (pts[0].x, pts[0].y));
            for p in &pts {
                (lo, hi) = ((lo.0.min(p.x), lo.1.min(p.y)), (hi.0.max(p.x), hi.1.max(p.y)));
            }
            for v in [contours as i32, lo.0, lo.1, hi.0, hi.1] {
                out.extend_from_slice(&(v as i16).to_be_bytes());
            }
            lo.0 as i16
        }
    };
    ends.iter().for_each(|e| out.extend_from_slice(&e.to_be_bytes()));
    out.extend_from_slice(&code_len.to_be_bytes());
    out.extend_from_slice(code);
    store_points(&pts, overlap, out);
    Some(x_min)
}
