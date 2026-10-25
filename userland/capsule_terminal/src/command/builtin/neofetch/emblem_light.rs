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

//! How the emblem is lit: the face from light at the upper left to deep at
//! the lower right, its edges that face the light caught, and the side in
//! shadow, every edge blended into the theme's background.

use super::emblem_cell::Rgb;
use super::emblem_shape::{cover_at, DEPTH};

// The face from light (upper left) to deep, its lit edges, and the side.
const TOP: Rgb = [196, 255, 246];
const BOT: Rgb = [28, 196, 184];
const LIT: Rgb = [240, 255, 252];
const NEAR: Rgb = [6, 92, 104];
const FAR: Rgb = [3, 26, 40];

// `a` toward `b` by `num` over `den`.
pub fn mix(a: Rgb, b: Rgb, num: u32, den: u32) -> Rgb {
    let m = |i: usize| (a[i] * (den - num) + b[i] * num + den / 2) / den;
    [m(0), m(1), m(2)]
}

/// Pixel `(x, y)` over `bg`, or None where it is the background.
pub fn pixel(x: i32, y: i32, bg: Rgb) -> Option<Rgb> {
    shade_at(x as f32, y as f32, (1.0, 1.0), bg)
}

/// The colour of the area `size` from `(fx, fy)`, in the emblem's pixels.
pub fn shade_at(fx: f32, fy: f32, size: (f32, f32), bg: Rgb) -> Option<Rgb> {
    let face = cover_at(fx, fy, size, 0.0);
    let (mut side, mut depth) = (0, 0);
    let steps = (DEPTH * 2.0) as i32;
    for step in 1..=steps {
        let s = cover_at(fx, fy, size, step as f32 / 2.0);
        if s > side {
            (side, depth) = (s, if depth == 0 { step } else { depth });
        }
    }
    if face == 0 && side == 0 {
        return None;
    }
    let lean = (6.0 * fx + 10.0 * fy).clamp(0.0, 444.0) as u32;
    let mut colour = mix(TOP, BOT, lean, 444);
    if face > 0 && cover_at(fx - 1.0, fy - 1.0, size, 0.0) < 8 {
        colour = mix(colour, LIT, 45, 100);
    }
    let under = mix(bg, mix(NEAR, FAR, depth.max(1) as u32 - 1, steps as u32 - 1), side, 16);
    Some(mix(under, colour, face, 16))
}
