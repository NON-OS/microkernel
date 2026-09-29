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

//! CSS gradients: integer table stepping stays within two levels of the
//! exact float gradient, and quarter turns run straight along an axis.
use crate::grad_fb::paint;

/* Channel error against 255 * t, t from the exact geometry of the box. */
fn worst(px: &[u32], w: usize, t: impl Fn(f32, f32) -> f32) -> u32 {
    let at = |i: usize| t((i % w) as f32, (i / w) as f32).clamp(0.0, 1.0);
    px.iter()
        .enumerate()
        .map(|(i, &p)| ((p & 0xff) as i32 - (255.0 * at(i)) as i32).unsigned_abs())
        .max()
        .unwrap()
}

#[test]
fn linear_and_radial_stay_within_two_levels_of_the_float_gradient() {
    let full = [0, 0, 400, 300];
    let px = paint(400, 300, "linear-gradient(30deg, #000000, #ffffff)", full, full);
    let (ux, uy) = (30f32.to_radians().sin(), -30f32.to_radians().cos());
    let lo = 300.0 * uy;
    let span = 400.0 * ux - lo;
    assert!(worst(&px, 400, |x, y| (x * ux + y * uy - lo) / span) <= 2);
    let px = paint(400, 300, "radial-gradient(#000000, #ffffff)", full, full);
    let r = (200f32 * 200.0 + 150.0 * 150.0).sqrt();
    assert!(worst(&px, 400, |x, y| ((x - 200.0).powi(2) + (y - 150.0).powi(2)).sqrt() / r) <= 2);
}

#[test]
fn quarter_turns_run_along_an_axis() {
    let full = [0, 0, 300, 120];
    let down = paint(300, 120, "linear-gradient(to bottom, #000000, #ffffff)", full, full);
    assert!(down.chunks(300).all(|row| row.iter().all(|&p| p == row[0])), "rows are flat");
    assert_eq!((down[0], down[300 * 120 - 1] & 0xff), (0xff00_0000, 252));
    let right = paint(300, 120, "linear-gradient(90deg, #000000, #ffffff)", full, full);
    assert!(right.chunks(300).all(|row| row == &right[..300]), "columns are flat");
}
