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

//! A gradient box only partly on screen paints exactly the rows that show,
//! translucent stops composite as before, and non-gradients are refused.
use nonos_app_skeleton::PaintBuffer;

use crate::grad_fb::paint;
use crate::shim::{is_gradient, paint_gradient};

#[test]
fn a_partly_visible_box_paints_the_rows_that_show() {
    let src = "linear-gradient(170deg, #102030, rgba(200, 100, 50, 0.5) 60%, #ffffff)";
    let whole = paint(64, 200, src, [0, 0, 64, 200], [0, 0, 64, 200]);
    let part = paint(64, 200, src, [0, -50, 64, 200], [0, 0, 64, 100]);
    assert_eq!(&part[..64 * 100], &whole[64 * 50..64 * 150]);
    assert!(part[64 * 100..].iter().all(|&p| p == 0xff00_00ff), "below the clip is untouched");
    let off = paint(64, 50, src, [0, 500, 64, 200], [0, 0, 64, 50]);
    assert!(off.iter().all(|&p| p == 0xff00_00ff), "a box below the view paints nothing");
}

#[test]
fn translucent_stops_composite_and_values_that_are_not_gradients_are_refused() {
    let full = [0, 0, 8, 8];
    let px = paint(8, 8, "linear-gradient(rgba(255, 0, 0, 0.5), rgba(255, 0, 0, 0.5))", full, full);
    assert!(px.iter().all(|&p| p == 0xff7f_0080), "{:08x}", px[0]);
    assert!(!is_gradient("url(a.png)") && is_gradient("radial-gradient(#fff, #000)"));
    let mut buf = [0u32; 4];
    let fb = &mut PaintBuffer { pixels: &mut buf, stride_words: 2, width: 2, height: 2 };
    assert!(!paint_gradient(fb, "linear-gradient(nonsense)", [0, 0, 2, 2], [0, 0, 2, 2]));
}
