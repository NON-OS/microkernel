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

use alloc::vec::Vec;

use crate::browser::image::webp::vp8l::decode_headless;

/// An ALPH chunk's `w` x `h` opacity plane: raw bytes or a lossless VP8L
/// stream (its green channel), then undone row by row from the filter the
/// encoder predicted with: none, horizontal, vertical or gradient.
pub(super) fn decode_alpha(chunk: &[u8], w: usize, h: usize) -> Option<Vec<u8>> {
    let (&head, data) = chunk.split_first()?;
    let (method, filter) = (head & 3, (head >> 2) & 3);
    if head >> 6 != 0 || method > 1 || (head >> 4) & 3 > 1 {
        return None;
    }
    let mut a: Vec<u8> = if method == 0 {
        data.get(..w.checked_mul(h)?)?.to_vec()
    } else {
        decode_headless(data, w, h)?.iter().map(|p| (p >> 8) as u8).collect()
    };
    for y in 0..h {
        let (above, row) = a.split_at_mut(y * w);
        let prev = (y > 0).then(|| &above[(y - 1) * w..]);
        unfilter(filter, prev, &mut row[..w]);
    }
    Some(a)
}

fn unfilter(filter: u8, prev: Option<&[u8]>, row: &mut [u8]) {
    match (filter, prev) {
        (0, _) => {}
        (2, Some(p)) => row.iter_mut().zip(p).for_each(|(v, &t)| *v = v.wrapping_add(t)),
        (3, Some(p)) => {
            let (mut left, mut corner) = (p[0], p[0]);
            for (i, v) in row.iter_mut().enumerate() {
                let top = p[i];
                let g = (left as i32 + top as i32 - corner as i32).clamp(0, 255) as u8;
                left = v.wrapping_add(g);
                *v = left;
                corner = top;
            }
        }
        (_, p) => {
            let mut pred = p.map_or(0, |p| p[0]);
            for v in row.iter_mut() {
                *v = v.wrapping_add(pred);
                pred = *v;
            }
        }
    }
}
