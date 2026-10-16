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

//! A window with a shadow, moved and raised, leaves no band of itself or its
//! shadow behind.

use super::foreign_desk::*;
use super::foreign_kernel::*;

/// An app window's shadow and frame are drawn inside its own surface, in a
/// translucent margin round the frame, so its layer rectangle holds all of
/// it. A window moved over another, and the other raised over it, must leave
/// no band of its shadow or frame anywhere, from the damage the submit and
/// the raise report alone (no extra commit from the app).
#[test]
fn a_moved_and_raised_shadowed_window_leaves_no_band() {
    const SHADOW: u32 = 0x6000_0000;
    let mut d = Desk::new();
    d.kernel.register(50, 40, 30, 40, 0xFF20_2830);
    d.submit(6, 50, (40, 20, 40, 30), 2);
    // Window 7: a frame inside a 3 pixel shadow margin.
    d.kernel.register(60, 36, 28, 36, SHADOW);
    if let Some((px, ..)) = d.kernel.memory.get_mut(&60) {
        for y in 3..25 {
            for x in 3..33 {
                px[(y * 36 + x) as usize] = WINDOW;
            }
        }
    }
    d.submit(7, 60, (10, 10, 36, 28), 2);
    d.frame();
    d.assert_current("opened");
    // Dragged in steps across window 6, a submit at each new place.
    for (x, y) in [(18, 14), (27, 19), (36, 24), (45, 28), (52, 30)] {
        d.submit(7, 60, (x, y, 36, 28), 2);
        d.frame();
        d.assert_current("dragged");
    }
    // Window 6 is clicked: the window manager raises it.
    if let Some(r) = crate::scene_raise::raise_by_pid(&mut d.scene, 6) {
        d.damage.accumulate(r);
    }
    d.frame();
    d.assert_current("the window under it raised");
}
