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

/* Where a w x h image goes in the box [x, y, box_w, box_h] under object-fit:
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
        ObjectFit::Fill => (dest, whole),
        /* Whole image into a centred rect that fits inside the box. */
        ObjectFit::Contain => {
            let (dw, dh) = if bw * ih <= bh * iw {
                (bw as i32, ((bw * ih) / iw) as i32)
            } else {
                (((bh * iw) / ih) as i32, bh as i32)
            };
            ([x + (box_w - dw) / 2, y + (box_h - dh) / 2, dw, dh], whole)
        }
        /* A centred crop of the image, with the box's aspect, into the box. */
        ObjectFit::Cover => {
            let src = if bw * ih >= bh * iw {
                let sh = (iw * bh) / bw;
                [0, ((ih - sh) / 2) as u32, w, (sh as u32).max(1)]
            } else {
                let sw = (ih * bw) / bh;
                [((iw - sw) / 2) as u32, 0, (sw as u32).max(1), h]
            };
            (dest, src)
        }
    })
}
