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

//! The emblem's colours, and its rows of half blocks (`emblem_shape`).
use alloc::vec::Vec;

use super::compose::Row;
use super::emblem_cell::{block, cell, Rgb};
use super::emblem_shape::{cover, DEPTH, EMBLEM_H, EMBLEM_W};

// The face from light (upper left) to deep, its lit edges, and the side.
const TOP: Rgb = [196, 255, 246];
const BOT: Rgb = [28, 196, 184];
const LIT: Rgb = [240, 255, 252];
const NEAR: Rgb = [6, 92, 104];
const FAR: Rgb = [3, 26, 40];

// `a` toward `b` by `num` over `den`.
fn mix(a: Rgb, b: Rgb, num: u32, den: u32) -> Rgb {
    let m = |i: usize| (a[i] * (den - num) + b[i] * num + den / 2) / den;
    [m(0), m(1), m(2)]
}

/// Pixel `(x, y)` over `bg`, or None where it is the background.
pub fn pixel(x: i32, y: i32, bg: Rgb) -> Option<Rgb> {
    let face = cover(x, y, 0.0);
    let (mut side, mut depth) = (0, 0);
    let steps = (DEPTH * 2.0) as i32;
    for step in 1..=steps {
        let s = cover(x, y, step as f32 / 2.0);
        if s > side {
            (side, depth) = (s, if depth == 0 { step } else { depth });
        }
    }
    if face == 0 && side == 0 {
        return None;
    }
    let mut colour = mix(TOP, BOT, (6 * x + 10 * y).clamp(0, 480) as u32, 480);
    if face > 0 && cover(x - 1, y - 1, 0.0) < 8 {
        colour = mix(colour, LIT, 45, 100);
    }
    let under = mix(bg, mix(NEAR, FAR, depth.max(1) as u32 - 1, steps as u32 - 1), side, 16);
    Some(mix(under, colour, face, 16))
}

/// The emblem's rows over the theme's background `bg` (0xRRGGBB); a file gets
/// the face alone in plain blocks.
pub fn emblem(bg: u32) -> Vec<Row> {
    let bg = [bg >> 16 & 0xFF, bg >> 8 & 0xFF, bg & 0xFF];
    let mut rows = Vec::new();
    for r in 0..(EMBLEM_H / 2) as i32 {
        let (mut plain, mut styled) = (Vec::new(), Vec::new());
        for x in 0..EMBLEM_W as i32 {
            let (up, down) = (pixel(x, 2 * r, bg), pixel(x, 2 * r + 1, bg));
            let (a, b) = (cover(x, 2 * r, 0.0) >= 8, cover(x, 2 * r + 1, 0.0) >= 8);
            plain.extend_from_slice(block(a, b));
            cell(&mut styled, up, down);
        }
        rows.push((plain, styled));
    }
    rows
}
