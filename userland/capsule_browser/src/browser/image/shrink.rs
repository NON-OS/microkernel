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

use super::store::Decoded;

/* Paths spelled out so the children resolve beside this file however it
 * is itself included (the host proofs include it by path). */
#[path = "shrink/fit_size.rs"]
mod fit_size;
#[path = "shrink/flush.rs"]
mod flush;
pub(super) use fit_size::fit;

/// A box filter fed one source row at a time, top down: every source
/// pixel lands in the target pixel covering it, averaged with alpha
/// weighting so transparent pixels lend no colour. Only the target raster
/// and one row of sums are held.
pub(super) struct Shrink {
    src: (usize, usize),
    dst: (usize, usize),
    sums: Vec<[u64; 5]>,
    px: Vec<u32>,
    row: usize,
}

impl Shrink {
    /// None when the target raster cannot be allocated.
    pub(super) fn new(src: (u32, u32), dst: (u32, u32)) -> Option<Shrink> {
        let (src, dst) =
            ((src.0 as usize, src.1 as usize), (dst.0.max(1) as usize, dst.1.max(1) as usize));
        let mut px = Vec::new();
        px.try_reserve_exact(dst.0.checked_mul(dst.1)?).ok()?;
        px.resize(dst.0 * dst.1, 0);
        Some(Shrink { src, dst, sums: alloc::vec![[0; 5]; dst.0], px, row: 0 })
    }

    pub(super) fn row(&mut self, y: usize, line: &[u32]) {
        let ty = (y * self.dst.1 / self.src.1.max(1)).min(self.dst.1 - 1);
        if ty != self.row {
            self.flush();
            self.row = ty;
        }
        for (x, &p) in line.iter().enumerate().take(self.src.0) {
            let s = &mut self.sums[(x * self.dst.0 / self.src.0).min(self.dst.0 - 1)];
            let a = (p >> 24) as u64;
            s[0] += ((p >> 16) & 0xFF) as u64 * a;
            s[1] += ((p >> 8) & 0xFF) as u64 * a;
            s[2] += (p & 0xFF) as u64 * a;
            s[3] += a;
            s[4] += 1;
        }
    }

    pub(super) fn finish(mut self) -> Decoded {
        self.flush();
        Decoded { w: self.dst.0 as u32, h: self.dst.1 as u32, px: self.px }
    }
}
