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
//! A shape is asked for well formed and set by its surface's owner alone,
//! kept when the window is submitted again, and sent to every cursor plane
//! when the window under the pointer wants another.

use super::frame_util::layer;
use crate::frame_pacer::cursor_shapes::{EW_CODE, NWSE_CODE};
use crate::requests::pointer::cursor_shape;
use crate::state::cursor_pick::shape_at;
use crate::state::scene::SceneTable;

const SYNC: &str = include_str!("../../../../compositor/src/frame_pacer/hw_cursor.rs");

#[test]
fn a_shape_is_asked_for_well_formed_and_set_by_its_surfaces_owner_alone() {
    let body = |code: u32, pad: u32| {
        [7u32, 0, code, pad].iter().flat_map(|w| w.to_le_bytes()).collect::<Vec<u8>>()
    };
    assert_eq!(cursor_shape(&body(3, 0)), Some((7, Some(NWSE_CODE))));
    assert_eq!(cursor_shape(&body(5, 0)), None, "a shape the compositor does not draw");
    assert_eq!(cursor_shape(&body(1, 1)), None, "the reserved word not zero");
    assert_eq!(cursor_shape(&body(1, 0)[..12]), None, "short");
    let mut scene = SceneTable::new();
    scene.submit(layer(40, 0, 0, 400, 300, 2)).unwrap();
    assert!(scene.set_cursor(41, 40, Some(EW_CODE)).is_none(), "another pid's surface");
    assert_eq!(scene.set_cursor(40, 40, Some(EW_CODE)).map(|l| l.cursor), Some(Some(EW_CODE)));
    scene.submit(layer(40, 10, 10, 500, 300, 2)).unwrap();
    let (kept, n) = scene.z_sorted_snapshot();
    assert_eq!(shape_at(kept[..n].iter(), 20, 20), EW_CODE, "kept when submitted again");
}

#[test]
fn a_new_shape_under_the_pointer_is_sent_to_every_cursor_plane() {
    let changed = SYNC.find("if ctx.hw_cursor() && want != ctx.cursor_code {").expect("compared");
    let owed = &SYNC[changed..];
    for line in ["ctx.cursor_code = want;", "ctx.cursor_sent = None;", "h.shaped = false"] {
        assert!(owed[..300].contains(line), "{line}");
    }
}
