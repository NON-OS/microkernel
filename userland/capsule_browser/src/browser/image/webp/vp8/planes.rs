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

/// One sample plane covering whole macroblocks.
pub(super) struct Plane {
    pub px: Vec<u8>,
    pub stride: usize,
}

impl Plane {
    /// None when the plane cannot be allocated.
    pub(super) fn new(w: usize, h: usize) -> Option<Plane> {
        let mut px = Vec::new();
        px.try_reserve_exact(w.checked_mul(h)?).ok()?;
        px.resize(w * h, 0);
        Some(Plane { px, stride: w })
    }

    /// A sample for prediction, with the frame's virtual border: 127 on
    /// the row above the frame (corner included), 129 left of it.
    pub(super) fn at(&self, x: isize, y: isize) -> u8 {
        if y < 0 {
            127
        } else if x < 0 {
            129
        } else {
            self.px[y as usize * self.stride + x as usize]
        }
    }

    /// `n` samples of row `y` from column `x`, bordered as `at` is.
    pub(super) fn row(&self, x: isize, y: isize, n: usize) -> [u8; 16] {
        let mut out = [0u8; 16];
        for (i, o) in out.iter_mut().take(n).enumerate() {
            *o = self.at(x + i as isize, y);
        }
        out
    }

    /// `n` samples of column `x` from row `y`.
    pub(super) fn col(&self, x: isize, y: isize, n: usize) -> [u8; 16] {
        let mut out = [0u8; 16];
        for (i, o) in out.iter_mut().take(n).enumerate() {
            *o = self.at(x, y + i as isize);
        }
        out
    }

    /// Write an n x n block at (x, y).
    pub(super) fn put(&mut self, x: usize, y: usize, n: usize, block: &[u8]) {
        for (r, src) in block.chunks(n).enumerate() {
            let at = (y + r) * self.stride + x;
            self.px[at..at + n].copy_from_slice(src);
        }
    }
}
