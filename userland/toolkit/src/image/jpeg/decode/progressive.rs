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

use crate::image::jpeg::coef::{cost, decode_scaled};
use crate::image::types::{DecodeError, ImageSize};

/* Coefficient storage a full-size progressive decode may take. */
const MAX_COEF_BYTES: usize = 32 * 1024 * 1024;

/// A progressive frame at full size into the caller's `out` (row-major,
/// its natural width), through the coefficient-buffered decoder.
pub fn progressive(input: &[u8], out: &mut [u32]) -> Result<ImageSize, DecodeError> {
    let c = cost(input)?;
    let w = c.w as usize;
    if out.len() < w * c.h as usize {
        return Err(DecodeError::OutputTooSmall);
    }
    let (ow, oh) = decode_scaled(input, 0, MAX_COEF_BYTES, &mut |y, row| {
        out[y * w..y * w + row.len()].copy_from_slice(row);
    })?;
    ImageSize::new(ow, oh)
}
