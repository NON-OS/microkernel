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

use crate::image::jpeg::coef::refuse::{MALFORMED, OTHER_PROCESS};
use crate::image::types::DecodeError;

mod comp;
pub use comp::Comp;

/// A baseline, extended (8-bit) or progressive frame header.
pub struct Frame {
    pub w: usize,
    pub h: usize,
    pub progressive: bool,
    pub n: usize,
    pub comps: [Comp; 4],
    pub hmax: usize,
    pub vmax: usize,
    pub mcux: usize,
    pub mcuy: usize,
}

/// Parse an SOF0, SOF1 or SOF2 segment body. Up to four components with
/// sampling factors 1..=4 that divide the largest (so chroma upsamples by
/// whole factors); a single component is always one block per MCU.
pub fn parse_frame(seg: &[u8], progressive: bool) -> Result<Frame, DecodeError> {
    let be = |i: usize| u16::from_be_bytes([seg[i], seg[i + 1]]) as usize;
    if seg.len() < 6 {
        return Err(DecodeError::Truncated);
    }
    let (h, w, n) = (be(1), be(3), seg[5] as usize);
    if seg[0] != 8 {
        return Err(OTHER_PROCESS);
    }
    if n == 0 || n > 4 {
        return Err(MALFORMED);
    }
    if w == 0 || h == 0 {
        return Err(DecodeError::BadDimensions);
    }
    if seg.len() < 6 + 3 * n {
        return Err(DecodeError::Truncated);
    }
    let comps = [Comp::default(); 4];
    let mut f = Frame { w, h, progressive, n, comps, hmax: 1, vmax: 1, mcux: 0, mcuy: 0 };
    for (i, c) in f.comps.iter_mut().take(n).enumerate() {
        let b = &seg[6 + 3 * i..9 + 3 * i];
        let (ch, cv) = if n == 1 { (1, 1) } else { ((b[1] >> 4) as usize, (b[1] & 15) as usize) };
        if !(1..=4).contains(&ch) || !(1..=4).contains(&cv) || b[2] > 3 {
            return Err(MALFORMED);
        }
        *c = Comp { id: b[0], h: ch, v: cv, tq: b[2] as usize, ..Comp::default() };
        (f.hmax, f.vmax) = (f.hmax.max(ch), f.vmax.max(cv));
    }
    comp::place(&mut f)?;
    Ok(f)
}
