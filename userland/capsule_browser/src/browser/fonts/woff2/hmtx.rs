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

use super::cursor::Cursor;

/// Rebuild hmtx from its transform (WOFF2 5.4): advance widths, then the
/// proportional and monospaced side bearings unless flag bit 0 or 1 says
/// they equal each glyph's xMin and were left out.
pub(super) fn rebuild(t: &[u8], metrics: usize, x_mins: &[i16]) -> Option<Vec<u8>> {
    let mut c = Cursor::new(t);
    let flags = c.u8()?;
    let glyphs = x_mins.len();
    if flags & 0xfc != 0 || flags & 3 == 0 || metrics == 0 || metrics > glyphs {
        return None;
    }
    let mut advances = Vec::new();
    advances.try_reserve_exact(metrics).ok()?;
    for _ in 0..metrics {
        advances.push(c.u16()?);
    }
    let mut out = Vec::new();
    out.try_reserve_exact(2 * (glyphs + metrics)).ok()?;
    for (i, &x_min) in x_mins.iter().enumerate() {
        let stored = if i < metrics { flags & 1 == 0 } else { flags & 2 == 0 };
        let lsb = if stored { c.u16()? as i16 } else { x_min };
        if let Some(advance) = advances.get(i) {
            out.extend_from_slice(&advance.to_be_bytes());
        }
        out.extend_from_slice(&lsb.to_be_bytes());
    }
    Some(out)
}
