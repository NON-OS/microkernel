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

//! The compositor may read a window's surface at any moment of a partial
//! repaint. While the app clears its content and draws it again, the surface
//! must still hold the previous frame; after, the new one, with the rows
//! outside the content as they were.

use super::off_screen::draw_off_screen;

const W: usize = 12;
const H: usize = 10;
const OLD: u32 = 0xFF11_2233;
const NEW: u32 = 0xFF44_5566;
const CLEAR: u32 = 0xFF00_0000;

#[test]
fn a_composite_during_a_partial_repaint_sees_the_old_content() {
    let mut surface = vec![OLD; W * H];
    let target = surface.as_ptr();
    let mut during = Vec::new();
    draw_off_screen(&mut surface, W, 3..8, |px| {
        // The app clears its content area, then draws it again.
        px[3 * W..8 * W].fill(CLEAR);
        during = unsafe { core::slice::from_raw_parts(target, W * H) }.to_vec();
        px[3 * W..8 * W].fill(NEW);
    });
    assert!(during.iter().all(|&p| p == OLD), "a composite read the content half drawn");
    assert!(surface[3 * W..8 * W].iter().all(|&p| p == NEW), "the new content is on the surface");
}

#[test]
fn rows_outside_the_content_are_not_written() {
    let mut surface = vec![OLD; W * H];
    draw_off_screen(&mut surface, W, 3..8, |px| px.fill(NEW));
    assert!(surface[..3 * W].iter().all(|&p| p == OLD), "the title bar's rows are left");
    assert!(surface[8 * W..].iter().all(|&p| p == OLD), "the bottom border's rows are left");
}
