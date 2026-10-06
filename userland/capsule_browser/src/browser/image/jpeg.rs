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

use nonos_toolkit::image::jpeg::coef::{cost, decode_scaled};
use nonos_toolkit::image::jpeg::decode::OTHER_PROCESS;
use nonos_toolkit::image::types::DecodeError;

use super::shrink::Shrink;
use super::store::Decoded;

/* Coefficient storage one decode may hold while its scans arrive. */
const MAX_COEF_BYTES: usize = 12 * 1024 * 1024;

/// A JPEG (baseline, extended or progressive) at `target` size: the IDCT
/// reads only as many coefficients as a 1/2, 1/4 or 1/8 decode needs while
/// that still covers the target (or while storage would pass its cap), and
/// the box filter takes the rows the rest of the way.
pub(super) fn decode_jpeg(bytes: &[u8], target: (u32, u32)) -> Result<Decoded, &'static str> {
    let c = cost(bytes).map_err(reason)?;
    let at = |s: usize| (c.w.div_ceil(1 << s), c.h.div_ceil(1 << s));
    let mut shift = 0;
    while shift < 3 && at(shift + 1).0 >= target.0 && at(shift + 1).1 >= target.1 {
        shift += 1;
    }
    while shift < 3 && c.coef[shift] > MAX_COEF_BYTES {
        shift += 1;
    }
    if c.coef[shift] > MAX_COEF_BYTES {
        return Err("jpeg too large even at 1/8");
    }
    let k = 8 >> shift;
    let src = ((c.w * k).div_ceil(8), (c.h * k).div_ceil(8));
    let dst = (target.0.min(src.0), target.1.min(src.1));
    let mut shrink = Shrink::new(src, dst).ok_or("no memory for the raster")?;
    decode_scaled(bytes, shift as u32, MAX_COEF_BYTES, &mut |y, row| shrink.row(y, row))
        .map_err(reason)?;
    Ok(shrink.finish())
}

/* Why a JPEG was turned away, in the words the page log shows. */
fn reason(e: DecodeError) -> &'static str {
    match e {
        OTHER_PROCESS => {
            "jpeg coding process not decoded here (lossless, arithmetic, hierarchical or 12-bit)"
        }
        DecodeError::BadMagic => "jpeg stream malformed",
        DecodeError::Truncated => "jpeg stream cut short before a whole scan",
        DecodeError::BadDimensions => "jpeg dimensions invalid or past the memory left",
        DecodeError::OutputTooSmall => "jpeg output buffer too small",
    }
}
