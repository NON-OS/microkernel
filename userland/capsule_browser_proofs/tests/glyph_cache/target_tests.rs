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

//! The clipped raster blit against blending each sample on its own, and the
//! glyph size limit.

use super::raster::{box_area, floor, Raster, MAX_GLYPH_AREA};
use super::target::Target;

fn sample() -> Raster {
    Raster { min_x: -1, min_y: -3, w: 5, h: 4, cov: (1..=20).map(|v| v * 12).collect() }
}

#[test]
fn blit_equals_blending_each_sample_at_every_offset() {
    let (r, stride, w, h) = (sample(), 9usize, 8u32, 6u32);
    for oy in -6..10 {
        for ox in -7..12 {
            /* The buffer ends five pixels into the last row, short of `w`. */
            let mut a = vec![0xff20_4060u32; stride * (h as usize - 1) + 5];
            let mut b = a.clone();
            let (bx, by) = (ox + r.min_x, oy + r.min_y);
            Target { buf: &mut a, stride, w, h, clip: [0, 0, w as i64, h as i64] }.blit(
                &r,
                bx,
                by,
                0xc0ff_8000,
            );
            let mut t = Target { buf: &mut b, stride, w, h, clip: [0, 0, w as i64, h as i64] };
            for y in 0..r.h {
                for x in 0..r.w {
                    let cov = r.cov[(y * r.w + x) as usize];
                    t.blend(bx + x as i32, by + y as i32, 0xc0ff_8000, cov);
                }
            }
            assert_eq!(a, b, "raster at ({ox}, {oy})");
        }
    }
}

#[test]
fn the_glyph_limit_is_the_cap_or_the_surface() {
    let mut small = vec![0u32; 4];
    assert_eq!(
        Target { buf: &mut small, stride: 2, w: 2, h: 2, clip: [0, 0, 2, 2] }.glyph_limit(),
        4
    );
    let mut none: Vec<u32> = Vec::new();
    let huge = Target { buf: &mut none, stride: 4096, w: 4096, h: 4096, clip: [0; 4] };
    assert_eq!(huge.glyph_limit(), MAX_GLYPH_AREA);
    assert_eq!(sample().area(), 20);
}

#[test]
fn box_area_saturates_and_floor_rounds_down() {
    assert_eq!(box_area(1024.0, 1024.0), MAX_GLYPH_AREA);
    assert_eq!(box_area(f32::INFINITY, 2.0), u64::MAX);
    assert_eq!(box_area(-3.0, 5.0), 0);
    for x in [-2.5f32, -2.0, -0.25, 0.0, 0.75, 3.0, 1.0e6 + 0.5] {
        assert_eq!(floor(x), x.floor(), "floor({x})");
    }
}
