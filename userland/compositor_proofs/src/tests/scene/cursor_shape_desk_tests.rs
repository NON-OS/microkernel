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
//! A window that asks for a resize arrow gets it drawn at the pointer, its
//! middle on the pointer's position, and moving it leaves nothing behind.

use super::foreign_desk::*;
use super::foreign_kernel::*;
use crate::frame_pacer::cursor::{FILL, OUTLINE};
use crate::frame_pacer::cursor_shapes::EW_CODE;

#[test]
fn the_window_s_resize_arrow_is_drawn_from_its_middle_and_leaves_no_trail() {
    let mut d = Desk::new();
    d.kernel.register(40, 60, 40, 60, WINDOW);
    d.submit(5, 40, (10, 10, 60, 40), 2);
    d.scene.set_cursor(5, 40, EW_CODE).expect("its own surface");
    d.frame();
    for step in 0..8u32 {
        let (x, y) = (30 + step * 3, 25 + step);
        d.move_cursor(x, y);
        d.frame();
        d.assert_current("the arrow moving over the window");
        let at = |px: u32, py: u32| d.canvas[(py * W + px) as usize];
        assert_eq!(at(x, y), FILL, "the middle on the pointer at step {step}");
        assert_eq!(at(x - 10, y), OUTLINE, "the left head's tip ten to the left");
    }
}
