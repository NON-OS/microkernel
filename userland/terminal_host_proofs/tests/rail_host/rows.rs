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

//! Row tiling, scrolled placement and the row label.

use crate::paint::rail_band::{clip, visible, Band};
use crate::paint::rail_left_geom::{nav_h, nav_sections};
use crate::paint::rail_row::{base_name, inside, row_band, row_h};
use crate::support::{list, RAIL};

#[test]
fn rows_tile_without_overlap_or_gap() {
    let l = Band { x: 12, y: 100, w: 216, h: 9 * row_h() };
    for i in 0..9 {
        let a = row_band(i, &l);
        assert_eq!(a.h, row_h(), "row {i} height");
        assert_eq!(a.x, l.x);
        assert_eq!(a.w, l.w);
        assert_eq!(a.y, l.y + (i * row_h()) as i32, "row {i} top");
        if i > 0 {
            let prev = row_band(i - 1, &l);
            assert_eq!(prev.y + prev.h as i32, a.y, "row {i} abuts row {}", i - 1);
        }
    }
}

#[test]
fn a_scrolled_row_keeps_its_place_in_the_column() {
    let rest = row_band(2, &list(0));
    let moved = row_band(2, &list(70));
    assert_eq!(moved.y, rest.y - 70, "the column translates, it does not relayout");
    assert_eq!(moved.h, rest.h);
    assert!(row_band(0, &list(1000)).y < 0, "a row above the viewport has a negative top");
    let gone = row_band(0, &list(1000));
    assert!(!visible(&gone, RAIL.h) && clip(gone.y, gone.h).is_none(), "scrolled out, skipped");
    assert!(visible(&rest, RAIL.h));
    assert_eq!(clip(rest.y, rest.h), Some((rest.y as u32, rest.h)), "a row in view is uncut");
    let g = nav_sections(RAIL, 70, 4, 3);
    assert_eq!(g.s_list.y, g.s_head.y + g.s_head.h as i32, "sessions sit under their caption");
    assert_eq!(g.p_list.y, g.p_head.y + g.p_head.h as i32, "projects sit under theirs");
    assert_eq!(g.p_list.y + g.p_list.h as i32 + 70, nav_h(4, 3) as i32, "the column's extent");
}

#[test]
fn base_name_takes_the_last_component() {
    assert_eq!(base_name("/home/user/src"), "src");
    assert_eq!(base_name("/"), "/");
    assert_eq!(base_name(""), "");
    assert_eq!(base_name("bare"), "bare");
    assert_eq!(base_name("/trailing/"), "/trailing/");
    assert!(inside(RAIL, RAIL.x, RAIL.y) && !inside(RAIL, RAIL.x + RAIL.w, RAIL.y));
}
