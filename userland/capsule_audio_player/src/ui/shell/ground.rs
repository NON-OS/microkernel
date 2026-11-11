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

//! The window ground: panel fill, the two ambient radials and the circuit
//! grid. Section 01 calls these part of the ground rather than decoration, so
//! they are baked once into a cache the size of the surface and copied in at
//! the head of every frame. Recomputing two elliptical radials per frame would
//! cost more than the rest of the paint put together.

extern crate alloc;

use alloc::vec::Vec;
use nonos_app_skeleton::PaintBuffer;
use nonos_toolkit::design::scale::scale;
use nonos_toolkit::theme::contrast::{at_least, BODY};
use nonos_toolkit::theme::derive::{mix, opaque};
use nonos_toolkit::theme::roles;

use super::super::metrics::{hair, px as scaled};

const GRID: i32 = 46;
/// How far, of 255, a grid line moves towards the accent.
const GRID_TINT: u8 = 5;

pub struct Ground {
    w: i32,
    h: i32,
    seeds: [u32; 3],
    q: u32,
    px: Vec<u32>,
}

/// The roles the ground is baked from, so a theme change rebuilds it.
fn seeds() -> [u32; 3] {
    let r = roles();
    [r.bg.argb(), r.surface.argb(), r.accent.argb()]
}

/// `base` tinted towards `tint` inside the ellipse of radii `rx` and `ry`, by
/// `peak` out of 255 at its centre, falling to nothing at its edge.
fn radial(base: u32, tint: u32, dx: i64, dy: i64, rx: i64, ry: i64, peak: u32) -> u32 {
    let d = (dx * dx * 10_000 / (rx * rx)) + (dy * dy * 10_000 / (ry * ry));
    if d >= 10_000 {
        return base;
    }
    let fall = (10_000 - d) as u32;
    mix(base, tint, (fall * peak / 10_000) as u8)
}

/// `peak`, or as much less as keeps text and muted text at the body floor
/// on a glow of it over either end of the gradient, grid line included. A
/// scheme whose text barely clears its surface takes a fainter glow.
fn readable(peak: u32, [bg, surface, accent]: [u32; 3]) -> u32 {
    let r = roles();
    let reads = |p: u32| {
        [bg, surface].iter().all(|&g| {
            let lit = mix(mix(g, accent, p as u8), accent, GRID_TINT);
            at_least(r.text.argb(), lit, BODY) && at_least(r.muted.argb(), lit, BODY)
        })
    };
    (0..=peak).rev().find(|&p| reads(p)).unwrap_or(0)
}

impl Ground {
    pub fn build(w: i32, h: i32) -> Ground {
        let mut px = Vec::with_capacity((w * h).max(0) as usize);
        let seeds @ [bg, surface, accent] = seeds();
        let (near, far) = (readable(16, seeds), readable(14, seeds));
        let (grid, hr) = (scaled(GRID), hair());
        let [rx, ry, fx, fy] = [550, 310, 380, 260].map(|n| scaled(n) as i64);
        for y in 0..h {
            for x in 0..w {
                let mut c = mix(bg, surface, (y * 255 / h.max(1)) as u8);
                c = radial(c, accent, (x - w / 2) as i64, (y + h / 10) as i64, rx, ry, near);
                c = radial(
                    c,
                    accent,
                    (x - w * 88 / 100) as i64,
                    (y - h * 108 / 100) as i64,
                    fx,
                    fy,
                    far,
                );
                if x % grid < hr || y % grid < hr {
                    c = mix(c, accent, GRID_TINT);
                }
                px.push(opaque(c));
            }
        }
        Ground { w, h, seeds, q: scale().quarters(), px }
    }

    pub fn fits(&self, w: u32, h: u32) -> bool {
        let same = (self.w, self.h, self.q) == (w as i32, h as i32, scale().quarters());
        same && self.seeds == seeds()
    }

    pub fn blit(&self, fb: &mut PaintBuffer) {
        let stride = fb.stride_words as usize;
        let w = (self.w as usize).min(fb.width as usize);
        for y in 0..(self.h as usize).min(fb.height as usize) {
            let dst = y * stride;
            let src = y * self.w as usize;
            if dst + w <= fb.pixels.len() {
                fb.pixels[dst..dst + w].copy_from_slice(&self.px[src..src + w]);
            }
        }
    }
}
