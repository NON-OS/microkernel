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

//! The copy a VRAM driver's FLUSH makes (section 12: "a VRAM device's an
//! anonymous shadow that FLUSH copies into mapped VRAM"): each rectangle
//! row by row from the shadow into the device's memory, swapping red and
//! blue for a framebuffer laid out red first. The caller fences the
//! write-combined mapping after it.

use nonos_gfx_wire::Rect;

/// Copy `rects` of a `src_stride` ARGB8888 shadow into `dst`, rows of
/// `dst_stride` bytes, 4 bytes a pixel. Rectangles were checked against
/// the shadow; any that would pass either buffer's end is skipped whole.
///
/// # Safety
/// `dst` must be 4-byte aligned and valid for writes of `dst_len` bytes
/// for the call.
pub unsafe fn copy_rects(
    src: &[u8],
    src_stride: usize,
    dst: *mut u8,
    dst_len: usize,
    dst_stride: usize,
    rects: &[Rect],
    swap_rb: bool,
) {
    if !(dst as usize).is_multiple_of(4) {
        return;
    }
    for r in rects {
        let (x, w) = (r.x as usize * 4, r.width as usize * 4);
        let rows = r.y as usize..r.y as usize + r.height as usize;
        let last = rows.end.saturating_sub(1);
        if last * src_stride + x + w > src.len() || last * dst_stride + x + w > dst_len {
            continue;
        }
        for y in rows {
            let s = &src[y * src_stride + x..y * src_stride + x + w];
            // SAFETY: the row lies within dst_len, checked above.
            let d = unsafe { dst.add(y * dst_stride + x) };
            for (i, px) in s.chunks_exact(4).enumerate() {
                let v = u32::from_le_bytes([px[0], px[1], px[2], px[3]]);
                let v = if swap_rb {
                    (v & 0xFF00_FF00) | ((v >> 16) & 0xFF) | ((v & 0xFF) << 16)
                } else {
                    v
                };
                // SAFETY: within the row checked above; VRAM wants volatile.
                unsafe { core::ptr::write_volatile(d.add(i * 4) as *mut u32, v.to_le()) };
            }
        }
    }
}
