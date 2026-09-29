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

use super::frame::Frame;
use crate::image::jpeg::coef::refuse::MALFORMED;
use crate::image::types::DecodeError;

/// A scan header: which frame components it codes (by index), their DC
/// and AC tables, the spectral band `ss..=se` and the successive
/// approximation bit positions `ah` (previous) and `al` (this scan).
pub struct Scan {
    pub n: usize,
    pub comp: [usize; 4],
    pub td: [usize; 4],
    pub ta: [usize; 4],
    pub ss: usize,
    pub se: usize,
    pub ah: u32,
    pub al: u32,
}

/// Parse an SOS segment body and check it is a legal scan for the frame:
/// a sequential scan covers the whole band; a progressive one is either a
/// DC scan or an AC band of one component.
pub fn parse_scan(seg: &[u8], f: &Frame) -> Result<Scan, DecodeError> {
    let n = *seg.first().ok_or(DecodeError::Truncated)? as usize;
    if n == 0 || n > f.n {
        return Err(MALFORMED);
    }
    if seg.len() < 4 + 2 * n {
        return Err(DecodeError::Truncated);
    }
    let mut s = Scan { n, comp: [0; 4], td: [0; 4], ta: [0; 4], ss: 0, se: 0, ah: 0, al: 0 };
    let mut units = 0;
    for i in 0..n {
        let (id, t) = (seg[1 + 2 * i], seg[2 + 2 * i]);
        let c = f.comps[..f.n].iter().position(|c| c.id == id).ok_or(MALFORMED)?;
        if s.comp[..i].contains(&c) || t >> 4 > 3 || t & 15 > 3 {
            return Err(MALFORMED);
        }
        (s.comp[i], s.td[i], s.ta[i]) = (c, (t >> 4) as usize, (t & 15) as usize);
        units += f.comps[c].h * f.comps[c].v;
    }
    let p = 1 + 2 * n;
    (s.ss, s.se, s.ah, s.al) =
        (seg[p] as usize, seg[p + 1] as usize, (seg[p + 2] >> 4) as u32, (seg[p + 2] & 15) as u32);
    let legal = if !f.progressive {
        s.ss == 0 && s.se == 63 && s.ah == 0 && s.al == 0
    } else if s.ss == 0 {
        s.se == 0
    } else {
        n == 1 && s.ss <= s.se && s.se <= 63
    };
    if !legal || s.ah > 13 || s.al > 13 || (n > 1 && units > 10) {
        return Err(MALFORMED);
    }
    Ok(s)
}
