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

//! Damage through the scale: a screen kept only by damaged rectangles, each
//! taken onto it by the real blit for its step (every shell step, and the
//! shrink), equals the whole canvas taken onto it afresh.

use crate::damage::{DamageAccumulator, Rect};
use crate::display::scale_rule::{canvas_for, STEPS};
use crate::sw_blitter::{downscale, scaled, Surface};
use crate::tests::scene::frame_model::Rng;

fn surface(px: &mut [u32], w: u32, h: u32) -> Surface {
    let byte_len = px.len() as u64 * 4;
    Surface { base_va: px.as_mut_ptr() as u64, stride: w * 4, width: w, height: h, byte_len }
}

/// `rect` of the canvas onto the screen: shrunk, or at `q` quarters.
fn show(canvas: &mut [u32], c: (u32, u32), screen: &mut [u32], s: (u32, u32), r: Rect, q: u32) {
    let (cv, sv) = (surface(canvas, c.0, c.1), surface(screen, s.0, s.1));
    let _ = if c.0 > s.0 { downscale(cv, sv, r) } else { scaled(cv, sv, r, q) };
}

fn session(s: (u32, u32), c: (u32, u32), q: u32, seed: u64) {
    let (mut rng, n) = (Rng::new(seed), (s.0 * s.1) as usize);
    let (mut canvas, mut screen): (Vec<u32>, _) = ((0..c.0 * c.1).collect(), vec![u32::MAX; n]);
    let whole = Rect { x: 0, y: 0, width: c.0, height: c.1 };
    show(&mut canvas, c, &mut screen, s, whole, q);
    let mut acc = DamageAccumulator::new();
    for frame in 0..40u32 {
        for _ in 0..1 + rng.below(4) {
            let (x, y) = (rng.below(c.0), rng.below(c.1));
            let r = Rect { x, y, width: 1 + rng.below(c.0 / 3), height: 1 + rng.below(c.1 / 3) };
            for py in y..r.bottom().min(c.1) {
                for px in x..r.right().min(c.0) {
                    canvas[(py * c.0 + px) as usize] = frame << 20 | py << 10 | px;
                }
            }
            acc.accumulate(r);
        }
        while let Some(r) = acc.drain() {
            show(&mut canvas, c, &mut screen, s, r, q);
        }
        let mut fresh = vec![u32::MAX; n];
        show(&mut canvas, c, &mut fresh, s, whole, q);
        assert!(screen == fresh, "{s:?} at {q} quarters, seed {seed} frame {frame}: stale");
    }
}

#[test]
fn every_scale_step_leaves_no_stale_screen_pixel() {
    for (seed, q) in (1..=8u64).flat_map(|s| STEPS.map(|q| (s, q))) {
        session((97, 61), canvas_for(97, 61, q), q, seed);
    }
}

#[test]
fn a_canvas_shrunk_onto_a_smaller_screen_leaves_none() {
    for seed in 1..=8u64 {
        session((53, 37), (97, 61), 4, seed);
    }
}
