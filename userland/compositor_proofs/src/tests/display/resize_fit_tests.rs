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

//! The held host size put through the wire's `fit`, as `setup::resize`
//! does: a range clamps and aligns, a list gives its largest mode inside.

use nonos_gfx_wire::body::caps::{MODE_RANGE, MODE_SET};
use nonos_gfx_wire::fit::fit;
use nonos_gfx_wire::{Caps, Mode};

#[test]
fn the_held_size_goes_through_fit() {
    let caps = Caps {
        flags: MODE_SET | MODE_RANGE,
        max_width: 2560,
        max_height: 1600,
        width_align: 8,
        surface_budget: 2560 * 1600 * 4,
        ..Caps::default()
    };
    assert_eq!(fit((1013, 705), (1024, 768), &caps, &[]), Some((1008, 705)));
    assert_eq!(fit((1024, 768), (1024, 768), &caps, &[]), None, "already there");
    let list = Caps { flags: MODE_SET, ..Caps::default() };
    let modes =
        [Mode { width: 1920, height: 1080, flags: 0 }, Mode { width: 1280, height: 720, flags: 0 }];
    assert_eq!(fit((1500, 900), (1920, 1080), &list, &modes), Some((1280, 720)));
}
