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

//! The pointer over content that changes under it leaves no stale block of
//! the old content behind.

use super::foreign_desk::*;
use super::foreign_kernel::*;
use crate::damage::Rect;

#[test]
fn the_cursor_over_changing_content_leaves_no_stale_block() {
    let mut d = Desk::new();
    d.kernel.register(40, 60, 40, 60, WINDOW);
    d.submit(5, 40, (10, 10, 60, 40), 2);
    d.frame();
    for step in 0..10u32 {
        // The app draws a new line under the pointer and commits it.
        let colour = 0xFF00_0000 | (step * 0x0011_1111);
        if let Some((px, ..)) = d.kernel.memory.get_mut(&40) {
            let row = 5 + step * 3;
            for y in row..row + 3 {
                for x in 0..60 {
                    px[(y * 60 + x) as usize] = colour;
                }
            }
            d.damage.accumulate(Rect { x: 10, y: 10 + row, width: 60, height: 3 });
        }
        d.move_cursor(14 + step * 4, 12 + step * 3);
        d.frame();
        d.assert_current("the pointer over a changing window");
    }
}
