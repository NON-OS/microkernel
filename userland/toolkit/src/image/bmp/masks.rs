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

use crate::image::types::DecodeError;

use super::le::le_u32;

/* Channel masks R, G, B, A for the direct-color depths of a form `readable`
 * accepted. BI_RGB has fixed layouts (5-5-5 for 16 bits, BGRA bytes for 32);
 * BI_BITFIELDS stores R, G and B right after the 40-byte header, with A
 * following when the header is V3 or later or the compression is
 * BI_ALPHABITFIELDS. Palette depths and 24-bit BGR need none. */
pub(super) fn masks(
    input: &[u8],
    dib: usize,
    compression: u32,
    bpp: usize,
) -> Result<[u32; 4], DecodeError> {
    Ok(match (compression, bpp) {
        (0, 16) => [0x7C00, 0x03E0, 0x001F, 0],
        (0, 32) => [0x00FF_0000, 0x0000_FF00, 0x0000_00FF, 0xFF00_0000],
        (0, _) => [0; 4],
        _ => {
            let a = if dib >= 56 || compression == 6 { le_u32(input, 66)? } else { 0 };
            [le_u32(input, 54)?, le_u32(input, 58)?, le_u32(input, 62)?, a]
        }
    })
}

/* One channel of a direct-color pixel scaled to eight bits, rounding a
 * narrower field to the nearest level; an absent mask reads as zero. */
pub(super) fn channel(v: u32, m: u32) -> u32 {
    if m == 0 {
        return 0;
    }
    let shift = m.trailing_zeros();
    let bits = 32 - (m >> shift).leading_zeros();
    let c = (v & m) >> shift;
    if bits >= 8 {
        c >> (bits - 8)
    } else {
        let max = (1u32 << bits) - 1;
        (c * 255 + max / 2) / max
    }
}
