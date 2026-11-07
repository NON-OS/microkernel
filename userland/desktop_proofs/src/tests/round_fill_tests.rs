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

//! A rounded fill drawn with the middle of each corner row as one run is
//! the fill drawn pixel by pixel: every size, radius and alpha here, on a
//! buffer already painted, clipped at its edges or not.

use nonos_toolkit::paint::radius;
use nonos_toolkit::paint::PaintBuffer;

const W: u32 = 96;
const H: u32 = 80;

fn base() -> Vec<u32> {
    (0..W * H).map(|i| 0xFF00_0000 | (i.wrapping_mul(2_654_435_761) >> 8)).collect()
}

/// The fill as it was drawn before: coverage worked at every column of a
/// corner row.
fn by_pixel(fb: &mut PaintBuffer, (x, y, w, h): (u32, u32, u32, u32), r: u32, c: u32) {
    if w == 0 || h == 0 {
        return;
    }
    let r = radius::clamp_radius(w, h, r);
    if r == 0 {
        fb.blend_rect(x, y, w, h, c);
        return;
    }
    let (a, rgb) = ((c >> 24) & 0xFF, c & 0x00FF_FFFF);
    for row in 0..h {
        if row >= r && row < h - r {
            fb.blend_rect(x, y + row, w, 1, c);
            continue;
        }
        for col in 0..w {
            let cov = radius::coverage(col, row, w, h, r);
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
fn the_fill_is_the_one_drawn_pixel_by_pixel() {
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
        (0, 0, 1, 1),
        (95, 79, 30, 30),
        (4, 4, 2, 50),
    ];
    for b in boxes {
        for r in [0, 1, 2, 3, 5, 8, 13, 16, 20, 27, 40, 1000] {
            for c in [0xFF20_4060, 0x8010_E0A0, 0x0300_0000, 0x5A00_0000, 0x00FF_FFFF] {
                let want = drawn(|fb| by_pixel(fb, b, r, c));
                let got = drawn(|fb| fb.fill_round(b.0, b.1, b.2, b.3, r, c));
                assert!(want == got, "box {b:?} radius {r} colour {c:08x}");
            }
        }
    }
}

#[test]
fn a_buffer_shorter_than_its_rows_is_filled_as_far_as_it_goes() {
    // A buffer whose last row stops part way: each pixel past it is skipped
    // alike.
    for r in [0, 4, 12] {
        let run = |f: &dyn Fn(&mut PaintBuffer)| {
            let mut px = base();
            px.truncate((W * (H - 1) + 17) as usize);
            let mut fb = PaintBuffer { pixels: &mut px, stride_words: W, width: W, height: H };
            f(&mut fb);
            px
        };
        let want = run(&|fb| by_pixel(fb, (2, 40, 90, 40), r, 0xFF11_2233));
        let got = run(&|fb| fb.fill_round(2, 40, 90, 40, r, 0xFF11_2233));
        assert!(want == got, "radius {r}");
    }
}
