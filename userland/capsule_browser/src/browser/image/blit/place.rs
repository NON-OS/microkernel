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

use crate::browser::css::ObjectFit;
use crate::browser::layout::boxmodel::rel;

/* Where a w x h image goes in the box [x, y, box_w, box_h] under object-fit
 * and object-position:
 * the destination rect and the source rect drawn into it. None for an empty
 * image or box. */
pub(super) fn place(
    fit: ObjectFit,
    dest: [i32; 4],
    w: u32,
    h: u32,
) -> Option<([i32; 4], [u32; 4])> {
    let [x, y, box_w, box_h] = dest;
    if w == 0 || h == 0 || box_w <= 0 || box_h <= 0 {
        return None;
    }
    let (iw, ih) = (w as u64, h as u64);
    let (bw, bh) = (box_w as u64, box_h as u64);
    let whole = [0, 0, w, h];
    Some(match fit {
        /* Whole image into the whole box, ignoring aspect ratio. */
        ObjectFit::Fill(_) => (dest, whole),
        /* Whole image into a rect that fits inside the box, placed by
         * object-position in the room left over. */
        ObjectFit::Contain(p) => {
            let (dw, dh) = if bw * ih <= bh * iw {
                (bw as i32, ((bw * ih) / iw) as i32)
            } else {
                (((bh * iw) / ih) as i32, bh as i32)
            };
            ([x + rel(p[0], box_w - dw), y + rel(p[1], box_h - dh), dw, dh], whole)
        }
        /* The image scaled to cover the box, placed by object-position (its
         * overflow is negative room), and the part inside the box drawn. */
        ObjectFit::Cover(p) => {
            let s = (bw as f64 / iw as f64).max(bh as f64 / ih as f64);
            let (sw, sh) = ((iw as f64 * s + 0.5) as i32, (ih as f64 * s + 0.5) as i32);
            let (dx, dy) = (rel(p[0], box_w - sw), rel(p[1], box_h - sh));
            let (cw, ch) =
                (((bw as f64 / s) as u32).clamp(1, w), ((bh as f64 / s) as u32).clamp(1, h));
            let x0 = ((-dx as f64 / s).max(0.0) as u32).min(w - cw);
            let y0 = ((-dy as f64 / s).max(0.0) as u32).min(h - ch);
            (dest, [x0, y0, cw, ch])
        }
    })
}
