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

use crate::image::jpeg::zigzag::ZIGZAG;
use crate::image::types::DecodeError;

mod zeroed;
pub use zeroed::block_bytes;
pub(super) use zeroed::zeroed;

/// One component's quantized coefficients. A scaled decode keeps only the
/// top-left `k` x `k` of each block (what a k-point IDCT reads); a
/// progressive one also keeps a bit per coefficient saying it is nonzero,
/// which refinement scans need even for the ones not kept.
pub struct Plane {
    pub k: usize,
    pub coef: Vec<i16>,
    mask: Vec<u64>,
}

impl Plane {
    pub fn new(bw: usize, bh: usize, k: usize, masked: bool) -> Result<Plane, DecodeError> {
        let blocks = bw.checked_mul(bh).ok_or(DecodeError::BadDimensions)?;
        let coef = zeroed(blocks.checked_mul(k * k).ok_or(DecodeError::BadDimensions)?)?;
        let mask = if masked && k < 8 { zeroed(blocks)? } else { Vec::new() };
        Ok(Plane { k, coef, mask })
    }

    /// Where zigzag position `z` of block `blk` lives, if it is kept.
    fn slot(&self, blk: usize, z: usize) -> Option<usize> {
        let n = ZIGZAG[z & 63];
        let (row, col) = (n / 8, n % 8);
        (row < self.k && col < self.k).then(|| blk * self.k * self.k + row * self.k + col)
    }

    pub fn get(&self, blk: usize, z: usize) -> i16 {
        self.slot(blk, z).and_then(|i| self.coef.get(i).copied()).unwrap_or(0)
    }

    pub fn set(&mut self, blk: usize, z: usize, v: i16) {
        match self.slot(blk, z) {
            Some(i) => self.coef.get_mut(i).into_iter().for_each(|c| *c = v),
            None if v != 0 => {
                if let Some(m) = self.mask.get_mut(blk) {
                    *m |= 1 << (z & 63);
                }
            }
            None => {}
        }
    }

    /// Whether the coefficient has been given a nonzero value so far.
    pub fn nonzero(&self, blk: usize, z: usize) -> bool {
        match self.slot(blk, z) {
            Some(i) => self.coef.get(i).is_some_and(|&c| c != 0),
            None => self.mask.get(blk).is_some_and(|m| m >> (z & 63) & 1 != 0),
        }
    }
}
