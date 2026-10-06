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

/* Move `rows` full-width pixel rows from `src_y` to `dst_y`, in the order
 * that never reads a row already overwritten. */
pub(super) fn blit_rows(fb: &mut PaintBuffer, src_y: u32, dst_y: u32, rows: u32) {
    let stride = fb.stride_words as usize;
    let w = fb.width as usize;
    let len = fb.pixels.len();
    let row = |y: u32| y as usize * stride;
    let copy = |px: &mut [u32], i: u32| {
        let (s, d) = (row(src_y + i), row(dst_y + i));
        if s + w <= len && d + w <= len {
            px.copy_within(s..s + w, d);
        }
    };
    if dst_y < src_y {
        (0..rows).for_each(|i| copy(fb.pixels, i));
    } else {
        (0..rows).rev().for_each(|i| copy(fb.pixels, i));
    }
}
