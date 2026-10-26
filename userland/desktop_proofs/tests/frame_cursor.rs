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
//! The pointer's shape over a frame (toolkit decorations/edges_cursor.rs):
//! a corner's diagonal, an edge's straight arrow, the arrow over none, and
//! each by the number the compositor draws it under.

use nonos_toolkit::decorations::{
    Edges, ARROW_CURSOR, EW_CURSOR, NESW_CURSOR, NS_CURSOR, NWSE_CURSOR,
};

const SHAPES: &str = include_str!("../../compositor/src/frame_pacer/cursor_shapes.rs");

fn e(left: bool, right: bool, top: bool, bottom: bool) -> Edges {
    Edges { left, right, top, bottom }
}

#[test]
fn each_edge_and_corner_shows_what_a_press_there_would_do() {
    let cases = [
        (e(false, false, false, false), ARROW_CURSOR),
        (e(true, false, false, false), EW_CURSOR),
        (e(false, true, false, false), EW_CURSOR),
        (e(false, false, true, false), NS_CURSOR),
        (e(false, false, false, true), NS_CURSOR),
        (e(true, false, true, false), NWSE_CURSOR),
        (e(false, true, false, true), NWSE_CURSOR),
        (e(false, true, true, false), NESW_CURSOR),
        (e(true, false, false, true), NESW_CURSOR),
    ];
    for (edges, want) in cases {
        assert_eq!(edges.cursor(), want, "{edges:?}");
    }
}

#[test]
fn each_shape_is_the_compositors_number_for_it() {
    for (name, ours) in [
        ("ARROW", ARROW_CURSOR),
        ("EW", EW_CURSOR),
        ("NS", NS_CURSOR),
        ("NWSE", NWSE_CURSOR),
        ("NESW", NESW_CURSOR),
    ] {
        let line = format!("pub const {name}_CODE: u8 = {ours};");
        assert!(SHAPES.contains(&line), "{line}");
    }
}
