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

use nonos_app_skeleton::PaintBuffer;

/* A source column or row for one destination column or row: the two texels
 * either side of the 8.8 fixed-point position `g` in an image `len` texels
 * long (clamped at the edge), and the weight of the second, 0..255. */
pub(super) fn taps(g: u32, len: u32) -> [u32; 3] {
    let i0 = (g >> 8).min(len - 1);
    [i0, (i0 + 1).min(len - 1), g & 0xff]
}

/* The four channels of an ARGB word in 16-bit lanes: B, R, G, A. */
fn spread(p: u32) -> u64 {
    (p & 0x00ff_00ff) as u64 | (((p & 0xff00_ff00) as u64) << 24)
}

/* Bilinear blend of texels c00, c10 (top) and c01, c11 (bottom) by (fx, fy),
 * per channel ((a(256-fx) + b fx)(256-fy) + (c(256-fx) + d fx) fy) >> 16:
 * the horizontal pass runs all four channels at once in 16-bit lanes, the
 * vertical pass two at a time in 32-bit lanes, so no lane can overflow. */
pub(super) fn bilinear(c: [u32; 4], fx: u32, fy: u32) -> u32 {
    let (ix, iy) = ((256 - fx) as u64, (256 - fy) as u64);
    let top = spread(c[0]) * ix + spread(c[1]) * fx as u64;
    let bot = spread(c[2]) * ix + spread(c[3]) * fx as u64;
    const L: u64 = 0x0000_ffff_0000_ffff;
    let half =
        |t: u64, b: u64| (((t & L) * iy + (b & L) * fy as u64) >> 16) & 0x0000_00ff_0000_00ff;
    let (bg, ra) = (half(top, bot), half(top >> 16, bot >> 16));
    (bg as u32) | ((ra as u32) << 16) | (((bg >> 32) as u32) << 8) | (((ra >> 32) as u32) << 24)
}

/* Source-over one sample; the fragment opacity scales the pixel's own alpha. */
pub(super) fn put(fb: &mut PaintBuffer, x: u32, y: u32, argb: u32, alpha: u8) {
    if x >= fb.width || y >= fb.height {
        return;
    }
    let idx = y as usize * fb.stride_words as usize + x as usize;
    if idx >= fb.pixels.len() {
        return;
    }
    let a = (((argb >> 24) & 0xff) * alpha as u32) / 255;
    if a == 0 {
        return;
    }
    if a == 255 {
        fb.pixels[idx] = 0xff00_0000 | (argb & 0x00ff_ffff);
        return;
    }
    let dst = fb.pixels[idx];
    let mix = |s: u32, d: u32| -> u32 { (s * a + d * (255 - a)) / 255 };
    let r = mix((argb >> 16) & 0xff, (dst >> 16) & 0xff);
    let g = mix((argb >> 8) & 0xff, (dst >> 8) & 0xff);
    let b = mix(argb & 0xff, dst & 0xff);
    fb.pixels[idx] = 0xff00_0000 | (r << 16) | (g << 8) | b;
}
