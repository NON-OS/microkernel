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

//! A window's frame clears only what its opaque panel does not overwrite
//! (toolkit decorations/frame_clear.rs): drawn over any old pixels, the
//! frame is the frame drawn over a cleared buffer, which is what the whole
//! clear before it gave, at every size, scale, state and theme listed.

use nonos_toolkit::decorations::{draw_frame_at, DecorationHit, FrameState};
use nonos_toolkit::paint::PaintBuffer;
use nonos_toolkit::theme::store::replace;
use nonos_toolkit::theme::{theme_of, SCHEMES};

const SIZES: [(u32, u32); 12] = [
    (1, 1),
    (8, 8),
    (24, 18),
    (40, 30),
    (63, 97),
    (120, 41),
    (200, 120),
    (320, 2000),
    (641, 33),
    (800, 600),
    (1280, 720),
    (1920, 1080),
];

/// The frame drawn into a `w` by `h` buffer `pad` words wider than it, every
/// word first `fill(i)`; the rows' first `w` words.
fn drawn(
    w: u32,
    h: u32,
    pad: u32,
    how: (bool, u32, FrameState),
    fill: fn(usize) -> u32,
) -> Vec<u32> {
    let stride = w + pad;
    let mut px: Vec<u32> = (0..(stride * h) as usize).map(fill).collect();
    let mut fb = PaintBuffer { pixels: &mut px, stride_words: stride, width: w, height: h };
    draw_frame_at(&mut fb, how.0, b"Terminal", how.2, 0, how.1);
    px.chunks(stride as usize).flat_map(|row| row[..w as usize].to_vec()).collect()
}

fn garbage(i: usize) -> u32 {
    (i as u32).wrapping_mul(0x9E37_79B9) | 0x0100_0000
}

/// Every case drawn over zeros and over old pixels, at the same stride; how
/// many cases differ. One test: the theme store is the process's.
#[test]
fn a_frame_over_old_pixels_is_the_frame_over_a_cleared_buffer_in_every_theme() {
    let states = [
        FrameState { hover: DecorationHit::None, focused: true },
        FrameState { hover: DecorationHit::CloseButton, focused: false },
        FrameState { hover: DecorationHit::MaximizeButton, focused: true },
    ];
    let mut cases = 0;
    for index in 0..SCHEMES.len() as u8 {
        for high in [false, true] {
            replace(theme_of(index, high));
            for (w, h) in SIZES {
                for maximized in [false, true] {
                    for quarters in [4u32, 5, 6, 8] {
                        for state in states {
                            let how = (maximized, quarters, state);
                            let clean = drawn(w, h, 3, how, |_| 0);
                            let over = drawn(w, h, 3, how, garbage);
                            let off = clean.iter().zip(&over).filter(|(a, b)| a != b).count();
                            assert_eq!(
                                off, 0,
                                "theme {index} high {high}, {w}x{h} maximized {maximized} at \
                                 {quarters}/4 {state:?}: {off} pixels kept what was there"
                            );
                            cases += 1;
                        }
                    }
                }
            }
        }
    }
    assert_eq!(cases, SCHEMES.len() * 2 * SIZES.len() * 2 * 4 * 3);
}

/// The time to draw a 1280x720 window's frame:
/// cargo test --release --test frame_clear frame_numbers -- --ignored --nocapture
#[test]
#[ignore]
fn frame_numbers() {
    use std::time::Instant;
    let state = FrameState { hover: DecorationHit::None, focused: true };
    let (w, h) = (1280u32, 720u32);
    let mut px = vec![0u32; (w * h) as usize];
    let mut t: Vec<f64> = (0..200)
        .map(|_| {
            let mut fb = PaintBuffer { pixels: &mut px, stride_words: w, width: w, height: h };
            let at = Instant::now();
            draw_frame_at(&mut fb, false, b"Terminal", state, 0, 4);
            at.elapsed().as_secs_f64() * 1e6
        })
        .collect();
    t.sort_by(f64::total_cmp);
    println!("frame 1280x720: p50 {:.0} us, p99 {:.0} us", t[100], t[198]);
}
