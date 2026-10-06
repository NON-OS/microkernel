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

use super::frame::Comp;
use super::idct::idct_scaled;
use super::plane::{zeroed, Plane};
use crate::image::types::DecodeError;

/// One component's samples at `k`/8 scale: every block of `plane`
/// dequantized with `q` and inverse-transformed into a plane `c.bw * k`
/// wide. The plane is reserved fallibly, as the coefficients were.
pub(super) fn idct_plane(
    c: &Comp,
    plane: &Plane,
    q: &[u16; 64],
    k: usize,
) -> Result<Vec<u8>, DecodeError> {
    let stride = c.bw * k;
    let n = stride.checked_mul(c.bh * k).ok_or(DecodeError::BadDimensions)?;
    let mut px: Vec<u8> = zeroed(n)?;
    for blk in 0..c.bw * c.bh {
        let at = (blk / c.bw) * k * stride + (blk % c.bw) * k;
        let src = &plane.coef[blk * k * k..(blk + 1) * k * k];
        idct_scaled(src, q, k, &mut px[at..], stride);
    }
    Ok(px)
}
