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

//! FLUSH (0x14): serial, count (1-15), then count rectangles of x, y,
//! width and height, u32 each. The rectangles are visible before the
//! reply; one outside the primary is EINVAL, a stale serial ESTALE.

use super::rect::Rect;
use crate::bytes::{put, u32_at};
use crate::errno::{Errno, EINVAL};

pub const MAX_RECTS: usize = 15;
const RECT_LEN: usize = 16;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Flush {
    pub serial: u32,
    pub rects: [Rect; MAX_RECTS],
    pub len: usize,
}

impl Flush {
    pub fn as_slice(&self) -> &[Rect] {
        &self.rects[..self.len]
    }

    pub fn encode(&self, out: &mut [u8]) -> Result<usize, Errno> {
        let n = self.len;
        if !(1..=MAX_RECTS).contains(&n) {
            return Err(EINVAL);
        }
        let mut ok =
            put(out, 0, &self.serial.to_le_bytes()) && put(out, 4, &(n as u32).to_le_bytes());
        for (i, r) in self.rects[..n].iter().enumerate() {
            for (j, v) in [r.x, r.y, r.width, r.height].iter().enumerate() {
                ok &= put(out, 8 + i * RECT_LEN + j * 4, &v.to_le_bytes());
            }
        }
        ok.then_some(8 + n * RECT_LEN).ok_or(EINVAL)
    }

    pub fn decode(b: &[u8]) -> Result<Self, Errno> {
        let n = u32_at(b, 4).ok_or(EINVAL)? as usize;
        if !(1..=MAX_RECTS).contains(&n) || b.len() < 8 + n * RECT_LEN {
            return Err(EINVAL);
        }
        let mut f = Flush { serial: u32_at(b, 0).ok_or(EINVAL)?, len: n, ..Flush::default() };
        for (i, r) in f.rects[..n].iter_mut().enumerate() {
            let g = |j: usize| u32_at(b, 8 + i * RECT_LEN + j * 4).ok_or(EINVAL);
            *r = Rect { x: g(0)?, y: g(1)?, width: g(2)?, height: g(3)? };
        }
        Ok(f)
    }
}
