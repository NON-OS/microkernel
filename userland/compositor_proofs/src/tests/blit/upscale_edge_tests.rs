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

//! The upscale at its edges: damage off the canvas clipped or nothing, a
//! scale of one a plain copy, and zero refused.

use super::upscale_buf::*;
use crate::damage::Rect;
use crate::sw_blitter::upscale;

#[test]
fn damage_hanging_off_the_canvas_is_clipped_and_damage_beyond_it_is_nothing() {
    let mut canvas = Buf::new(4, 3, distinct);
    let mut screen = Buf::new(8, 6, |_, _| 0);
    let hanging = Rect { x: 3, y: 2, width: 50, height: 50 };
    let out = upscale(canvas.surface(), screen.surface(), hanging, 2);
    assert_eq!(t(out), Some((6, 4, 2, 2)));
    assert_eq!(screen.at(7, 5), canvas.at(3, 2));
    let beyond = Rect { x: 4, y: 0, width: 1, height: 1 };
    assert_eq!(t(upscale(canvas.surface(), screen.surface(), beyond, 2)), None);
    let empty = Rect { x: 0, y: 0, width: 0, height: 1 };
    assert_eq!(t(upscale(canvas.surface(), screen.surface(), empty, 2)), None);
    assert!(screen.guards_intact());
}

#[test]
fn a_scale_of_one_is_a_plain_copy_and_zero_is_refused() {
    let mut canvas = Buf::new(4, 3, distinct);
    let mut screen = Buf::new(4, 3, |_, _| 0);
    let all = Rect { x: 0, y: 0, width: 4, height: 3 };
    assert_eq!(t(upscale(canvas.surface(), screen.surface(), all, 1)), Some((0, 0, 4, 3)));
    assert!((0..3).all(|y| (0..4).all(|x| screen.at(x, y) == canvas.at(x, y))));
    assert_eq!(t(upscale(canvas.surface(), screen.surface(), all, 0)), None);
}
