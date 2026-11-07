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

//! Nearest-pixel scaling, a row at a time. The source column of each
//! destination column is worked out once per frame, not once per pixel.

use alloc::vec::Vec;

use crate::scalar::load;

/// The source column of each destination column: the one under its centre.
pub struct NearestCols {
    at: Vec<u32>,
}

impl NearestCols {
    /// Columns for `to` destination pixels from `from` source pixels; None
    /// for an empty side or when there is no memory for the table.
    pub fn new(from: u32, to: u32) -> Option<Self> {
        let (f, t) = (u64::from(from), u64::from(to));
        if f == 0 || t == 0 {
            return None;
        }
        let mut at = Vec::new();
        at.try_reserve_exact(to as usize).ok()?;
        at.extend((0..t).map(|d| ((2 * d + 1) * f / (2 * t)).min(f - 1) as u32));
        Some(NearestCols { at })
    }

    /// The source column under destination column `d`.
    pub fn col(&self, d: usize) -> Option<u32> {
        self.at.get(d).copied()
    }
}

/// `dst` (whole pixels) from one source row, its first pixel at
/// destination column `from`. A column past the end of `src` or of the
/// table is left as it was.
pub fn nearest_row(dst: &mut [u8], src: &[u8], cols: &NearestCols, from: usize) {
    let at = cols.at.get(from..).unwrap_or(&[]);
    for (d, &c) in dst.chunks_exact_mut(4).zip(at) {
        let i = c as usize * 4;
        if let Some(s) = src.get(i..i + 4) {
            d.copy_from_slice(&load(s).to_le_bytes());
        }
    }
}
