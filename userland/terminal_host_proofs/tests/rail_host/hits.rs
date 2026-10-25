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

//! Hit-testing the navigation rows and captions.

use crate::paint::rail_left_geom::{hit, nav_sections, LeftHit};
use crate::paint::rail_row::{row_at, row_band, row_h};
use crate::support::{list, RAIL};

#[test]
fn hit_test_agrees_with_the_painter_at_every_index_and_offset() {
    for off in [0u32, 17, 60, 200] {
        let l = list(off);
        for i in 0..4 {
            let r = row_band(i, &l);
            for (x, y) in [
                (r.x, r.y),
                (r.x + r.w - 1, r.y),
                (r.x, r.y + r.h as i32 - 1),
                (r.x + r.w / 2, r.y + r.h as i32 / 2),
            ] {
                assert_eq!(row_at(&l, 4, x, y), Some(i), "off {off} row {i} at ({x},{y})");
            }
        }
    }
}

#[test]
fn a_point_outside_the_list_hits_no_row() {
    let l = list(0);
    assert_eq!(row_at(&l, 9, l.x, l.y - 1), None);
    assert_eq!(row_at(&l, 9, l.x + l.w, l.y), None);
    assert_eq!(row_at(&l, 9, l.x, l.y + (9 * row_h()) as i32), None);
}

#[test]
fn a_row_past_the_count_is_not_clickable() {
    let l = list(0);
    let r = row_band(2, &l);
    assert_eq!(row_at(&l, 2, r.x + 1, r.y + 1), None);
    assert_eq!(row_at(&l, 3, r.x + 1, r.y + 1), Some(2));
}

#[test]
fn hit_routes_captions_before_rows_and_follows_the_offset() {
    for off in [0u32, 40] {
        let g = nav_sections(RAIL, off, 4, 2);
        let plus = g.s_plus;
        let (px, py) = (RAIL.x + plus.x + plus.w / 2, (RAIL.y as i32 + plus.y + 1) as u32);
        assert!(matches!(hit(RAIL, 4, 2, off, px, py), Some(LeftHit::NewSession)), "off {off}");
        let a = row_band(1, &g.s_list);
        let ay = (RAIL.y as i32 + a.y + 4) as u32;
        assert!(matches!(hit(RAIL, 4, 2, off, RAIL.x + a.x + 4, ay), Some(LeftHit::Session(1))));
        let b = row_band(1, &g.p_list);
        let by = (RAIL.y as i32 + b.y + 4) as u32;
        assert!(matches!(hit(RAIL, 4, 2, off, RAIL.x + b.x + 4, by), Some(LeftHit::Project(1))));
    }
}
