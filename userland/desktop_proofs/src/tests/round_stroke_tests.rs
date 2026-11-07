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

//! A rounded outline drawn with its straight run between the corner curves
//! as one span is the outline drawn pixel by pixel: every size, radius,
//! thickness and alpha here, on a buffer already painted, and the shadows
//! a window frame and the dock draw from rings of it.

use nonos_toolkit::paint::radius;
use nonos_toolkit::paint::PaintBuffer;

const W: u32 = 96;
const H: u32 = 80;

fn base() -> Vec<u32> {
    (0..W * H).map(|i| 0xFF00_0000 | (i.wrapping_mul(2_654_435_761) >> 8)).collect()
}

/// The outline as it was drawn before: coverage worked at every pixel of a
/// corner row.
fn by_pixel(fb: &mut PaintBuffer, (x, y, w, h): (u32, u32, u32, u32), r: u32, t: u32, c: u32) {
    if t == 0 || w == 0 || h == 0 {
        return;
    }
    if w <= t * 2 || h <= t * 2 {
        fb.fill_round(x, y, w, h, r, c);
        return;
    }
    let r = radius::clamp_radius(w, h, r);
    let (iw, ih) = (w - t * 2, h - t * 2);
    let ir = radius::clamp_radius(iw, ih, r.saturating_sub(t));
    let (a, rgb) = ((c >> 24) & 0xFF, c & 0x00FF_FFFF);
    for row in 0..h {
        if row >= r && row < h - r {
            fb.blend_rect(x, y + row, t, 1, c);
            fb.blend_rect(x + w - t, y + row, t, 1, c);
            continue;
        }
        for col in 0..w {
            let outer = radius::coverage(col, row, w, h, r);
            if outer == 0 {
                continue;
            }
            let inner = if row >= t && row < h - t && col >= t && col < w - t {
                radius::coverage(col - t, row - t, iw, ih, ir)
            } else {
                0
            };
            let cov = outer.saturating_sub(inner);
            if cov != 0 {
                fb.blend_px(x + col, y + row, ((a * cov / 255) << 24) | rgb);
            }
        }
    }
}

fn drawn(f: impl FnOnce(&mut PaintBuffer)) -> Vec<u32> {
    let mut px = base();
    let mut fb = PaintBuffer { pixels: &mut px, stride_words: W, width: W, height: H };
    f(&mut fb);
    px
}

#[test]
fn the_outline_is_the_one_drawn_pixel_by_pixel() {
    let boxes = [
        (3, 2, 60, 40),
        (0, 0, W, H),
        (10, 7, 9, 31),
        (50, 40, 70, 60),
        (5, 5, 4, 4),
        (2, 3, 90, 70),
        (0, 0, 33, 33),
        (7, 1, 41, 17),
        (1, 9, 80, 12),
    ];
    for b in boxes {
        for r in [0, 1, 2, 3, 5, 8, 13, 16, 20, 27, 40] {
            for t in [1, 2, 3, 4, 6, 9, 15] {
                for c in [0xFF20_4060, 0x8010_E0A0, 0x0300_0000, 0x5A00_0000] {
                    let want = drawn(|fb| by_pixel(fb, b, r, t, c));
                    let got = drawn(|fb| fb.stroke_round(b.0, b.1, b.2, b.3, r, t, c));
                    assert!(want == got, "box {b:?} radius {r} thickness {t} colour {c:08x}");
                }
            }
        }
    }
}

#[test]
fn a_frames_shadow_is_the_one_drawn_pixel_by_pixel() {
    for (spread, r) in [(3, 16), (6, 16), (12, 10), (24, 12), (18, 0), (9, 30)] {
        let (x, y, w, h) = (28, 26, 40, 30);
        let got = drawn(|fb| fb.shadow_round(x, y, w, h, r, spread, 0x5A00_0000));
        let want = drawn(|fb| {
            let denom = (spread + 1) * (spread + 1);
            for i in 1..=spread {
                let step = spread + 1 - i;
                let ea = 0x5A * step * step / denom;
                if ea == 0 {
                    continue;
                }
                let (ox, oy) = (x.saturating_sub(i), y.saturating_sub(i));
                let b = (ox, oy, w + (x - ox) + i, h + (y - oy) + i);
                by_pixel(fb, b, r + i, 1, ea << 24);
            }
        });
        assert!(want == got, "spread {spread} radius {r}");
    }
}
