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

//! wp_cursor_shape_v1's shapes as the compositor's five drawings.

use crate::wayland_cursor_code::{code_of, ARROW, EW, NESW, NS, NWSE};

#[test]
fn every_resize_shape_is_its_axis_arrow() {
    for (shapes, code) in [
        (&[18, 25, 26, 30][..], EW),
        (&[19, 22, 27, 31][..], NS),
        (&[21, 23, 29][..], NWSE),
        (&[20, 24, 28][..], NESW),
    ] {
        for &s in shapes {
            assert_eq!(code_of(s), code, "shape {s}");
        }
    }
}

#[test]
fn every_other_shape_is_the_arrow() {
    let resize = 18..=31;
    for s in (0..=40).filter(|s| !resize.contains(s)) {
        assert_eq!(code_of(s), ARROW, "shape {s}");
    }
}
