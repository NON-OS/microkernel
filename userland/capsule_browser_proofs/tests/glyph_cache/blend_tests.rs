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

//! The integer glyph blend against the per-pixel float blend it replaced.

use super::blend::mix;

/* The float blend the toolkit drew glyphs with before, for one channel. */
fn float_mix(s: u32, d: u32, cov: u32, alpha: u32) -> u32 {
    let a = cov as f32 / 255.0 * alpha as f32 / 255.0;
    (s as f32 * a + d as f32 * (1.0 - a) + 0.5) as u32
}

/* Red carries (s over d), blue (d over s), green (255 - s over 255 - d). */
fn pair(alpha: u32, s: u32, d: u32) -> (u32, u32, [(u32, u32); 3]) {
    let src = alpha << 24 | s << 16 | (255 - s) << 8 | d;
    let dst = 0xff00_0000 | d << 16 | (255 - d) << 8 | s;
    (src, dst, [(s, d), (255 - s, 255 - d), (d, s)])
}

fn channels(p: u32) -> [u32; 3] {
    [p >> 16 & 0xff, p >> 8 & 0xff, p & 0xff]
}

#[test]
fn opaque_colours_round_every_input_to_nearest() {
    for cov in 0..=255u32 {
        for s in 0..=255u32 {
            for d in 0..=255u32 {
                let (src, dst, want) = pair(0xff, s, d);
                let got = channels(mix(dst, src, cov as u8));
                for (g, (s, d)) in got.into_iter().zip(want) {
                    assert_eq!(g, (s * cov + d * (255 - cov) + 127) / 255, "{s} {d} {cov}");
                }
            }
        }
    }
}

#[test]
fn every_colour_matches_the_float_blend_it_replaced() {
    let mut near_half = 0u32;
    for alpha in 0..=255u32 {
        for cov in 0..=255u32 {
            for s in 0..=255u32 {
                for d in (0..=255u32).step_by(17) {
                    let (src, dst, want) = pair(alpha, s, d);
                    let got = channels(mix(dst, src, cov as u8));
                    for (g, (s, d)) in got.into_iter().zip(want) {
                        assert_eq!(g, float_mix(s, d, cov, alpha), "{alpha} {cov} {s} {d}");
                        let k = (s * cov * alpha + d * (65025 - cov * alpha)) % 65025;
                        near_half += (alpha != 255 && (32511..=32513).contains(&k)) as u32;
                    }
                }
            }
        }
    }
    assert!(near_half > 0, "the sweep reached no input within 2/65025 of a half");
    assert_eq!(mix(0xff10_2030, 0xff12_3456, 255), 0xff12_3456);
    assert_eq!(mix(0xff10_2030, 0xff12_3456, 0), 0xff10_2030);
}
