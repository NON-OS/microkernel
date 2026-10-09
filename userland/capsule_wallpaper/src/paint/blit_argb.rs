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

use nonos_toolkit::image::scale::scale_cover;

// Fill the backing surface with `src`, scaled to cover it at the image's own
// aspect: area-averaged when shrinking, bilinear when enlarging, exact at 1:1.
pub fn blit_argb(
    backing_va: u64,
    stride_bytes: u32,
    backing_w: u32,
    backing_h: u32,
    src: &[u32],
    src_w: u32,
    src_h: u32,
) -> bool {
    let stride_px = (stride_bytes / 4) as usize;
    if stride_px < backing_w as usize {
        return false;
    }
    let dst = backing_va as *mut u32;
    scale_cover(src, src_w, src_h, backing_w, backing_h, |y, row| {
        let at = y as usize * stride_px;
        for (x, &p) in row.iter().enumerate() {
            // SAFETY: the backing is `stride_bytes * backing_h` bytes, mapped
            // read-write by `prime::backing::allocate`; `y < backing_h` and
            // `x < backing_w <= stride_px`, checked above, so the write is inside it.
            unsafe { core::ptr::write_volatile(dst.add(at + x), p) };
        }
    })
}
