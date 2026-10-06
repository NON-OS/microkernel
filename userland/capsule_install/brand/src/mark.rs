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

//! The NØNOS mark, from the brand kit's Ø (assets/mark-*.nxm), lit in cyan
//! with its glow, or unlit in grey; and the brand frame around it.

use nonos_app_skeleton::PaintBuffer;

use super::pixels::{dim, mix};

static MARKS: [(u32, &[u8], &[u8]); 3] = [
    (56, include_bytes!("../assets/mark-56.nxm"), include_bytes!("../assets/mark-56-glow.nxm")),
    (96, include_bytes!("../assets/mark-96.nxm"), include_bytes!("../assets/mark-96-glow.nxm")),
    (150, include_bytes!("../assets/mark-150.nxm"), include_bytes!("../assets/mark-150-glow.nxm")),
];

/// The largest mark no taller than `max_h`; its size, its Ø and its glow.
fn pick(max_h: u32) -> (u32, &'static [u8], &'static [u8]) {
    let mut best = MARKS[0];
    for m in MARKS {
        if m.0 <= max_h {
            best = m;
        }
    }
    best
}

/// Draw the Ø centred on (cx, cy), at most `max_h` tall, in `argb`; the
/// glow at `glow` of 256.
pub fn mark(fb: &mut PaintBuffer, cx: u32, cy: u32, max_h: u32, argb: u32, glow: u32) {
    let (_, m, g) = pick(max_h);
    if glow > 0 {
        stamp(fb, g, cx, cy, argb, glow);
    }
    stamp(fb, m, cx, cy, argb, 256);
}

/// Blend coverage `nxm` centred on (cx, cy) toward `argb` at `gain` / 256.
fn stamp(fb: &mut PaintBuffer, nxm: &[u8], cx: u32, cy: u32, argb: u32, gain: u32) {
    let (Some(w), Some(h)) = (dim(nxm, 4), dim(nxm, 6)) else { return };
    let (x0, y0) = (cx as i64 - w as i64 / 2, cy as i64 - h as i64 / 2);
    for y in 0..h {
        for x in 0..w {
            let a = nxm.get(8 + (y * w + x) as usize).copied().unwrap_or(0) as u32 * gain / 256;
            let (px, py) = (x0 + x as i64, y0 + y as i64);
            if a == 0 || px < 0 || py < 0 || px >= fb.width as i64 || py >= fb.height as i64 {
                continue;
            }
            let i = py as usize * fb.stride_words as usize + px as usize;
            if let Some(p) = fb.pixels.get_mut(i) {
                *p = mix(*p, argb, a.min(255));
            }
        }
    }
}
