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

use super::frame::parse_frame;
use super::marker::next_marker;
use super::plane::block_bytes;
use crate::image::jpeg::coef::refuse::OTHER_PROCESS;
use crate::image::types::DecodeError;

/// What a decode would need, read from the frame header alone: the
/// natural size and the coefficient bytes at each shift 0..=3.
pub struct Cost {
    pub w: u32,
    pub h: u32,
    pub coef: [usize; 4],
}

pub fn cost(input: &[u8]) -> Result<Cost, DecodeError> {
    let mut pos = 2;
    while let Some((m, p)) = next_marker(input, pos) {
        let len = input.get(p..p + 2).ok_or(DecodeError::Truncated)?;
        let len = (u16::from_be_bytes([len[0], len[1]]) as usize).max(2);
        let seg = input.get(p + 2..p + len).ok_or(DecodeError::Truncated)?;
        if matches!(m, 0xC0..=0xC2) {
            let f = parse_frame(seg, m == 0xC2)?;
            let blocks = f.comps[..f.n].iter().map(|c| c.bw * c.bh).sum::<usize>();
            let coef = [0, 1, 2, 3].map(|s| blocks * block_bytes(8 >> s, f.progressive));
            return Ok(Cost { w: f.w as u32, h: f.h as u32, coef });
        }
        if matches!(m, 0xC3..=0xCF) && m != 0xC4 && m != 0xC8 && m != 0xCC {
            return Err(OTHER_PROCESS);
        }
        pos = p + len;
    }
    Err(DecodeError::Truncated)
}
