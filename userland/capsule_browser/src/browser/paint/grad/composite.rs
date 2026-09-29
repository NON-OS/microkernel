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

/* One run of gradient pixels onto framebuffer row `y` from column `x`; a run
 * with no translucent pixel is a straight copy. */
pub(super) fn composite(fb: &mut PaintBuffer, y: i32, x: i32, src: &[u32]) {
    let at = y as usize * fb.stride_words as usize + x as usize;
    let Some(dst) = fb.pixels.get_mut(at..at + src.len()) else { return };
    if src.iter().all(|&s| s >= 0xff00_0000) {
        dst.copy_from_slice(src);
        return;
    }
    for (d, &s) in dst.iter_mut().zip(src) {
        *d = over(*d, s);
    }
}

/* `argb` composited source-over onto the opaque pixel `dst`: each channel is
 * floor((s * a + d * (255 - a)) / 255), red and blue side by side in one
 * word, the division done as (x + 1 + (x >> 8)) >> 8, exact for x < 65536. */
pub(super) fn over(dst: u32, argb: u32) -> u32 {
    let a = argb >> 24;
    if a == 0 {
        return dst;
    }
    if a == 255 {
        return argb;
    }
    let rb = (argb & 0x00ff_00ff) * a + (dst & 0x00ff_00ff) * (255 - a);
    let g = ((argb & 0xff00) * a + (dst & 0xff00) * (255 - a)) >> 8;
    let rb = ((rb + 0x0001_0001 + ((rb >> 8) & 0x00ff_00ff)) >> 8) & 0x00ff_00ff;
    0xff00_0000 | rb | (((g + 1 + (g >> 8)) >> 8) << 8)
}
