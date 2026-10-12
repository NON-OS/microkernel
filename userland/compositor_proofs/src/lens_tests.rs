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

//! The magnifier's lens: its power from the output's scale and the chosen
//! step, centred on the point it follows, and held inside the canvas at the
//! edges, so the lens never shows past the canvas while there is canvas.

use crate::display::magnifier_rule::{lens, Lens};

#[test]
fn the_power_is_the_scale_times_the_step() {
    assert_eq!(lens((960, 540), (1920, 1080), (1920, 1080), 8).z, 8, "2x on a 1x screen");
    assert_eq!(lens((480, 270), (960, 540), (1920, 1080), 8).z, 16, "2x on a doubled screen");
    assert_eq!(lens((0, 0), (1920, 1080), (1920, 1080), 4).z, 4, "off: one to one");
}

#[test]
fn centred_on_the_point_and_held_at_the_edges() {
    let screen = (1920, 1080);
    let l = lens((960, 540), (1920, 1080), screen, 8);
    assert_eq!(l, Lens { z: 8, ox: 480, oy: 270 }, "half the canvas, centred");
    assert_eq!(lens((10, 10), (1920, 1080), screen, 8), Lens { z: 8, ox: 0, oy: 0 });
    let far = lens((1919, 1079), (1920, 1080), screen, 8);
    assert_eq!((far.ox, far.oy), (960, 540), "the far corner, no past it");
    for x in (0..1920).step_by(37) {
        let l = lens((x, 500), (1920, 1080), screen, 12);
        let seen = (1920 * 4u32).div_ceil(12);
        assert!(l.ox + seen <= 1920 + 1, "x {x}: {l:?}");
        assert!(l.ox <= x && x < l.ox + seen, "x {x} is in the lens: {l:?}");
    }
}
