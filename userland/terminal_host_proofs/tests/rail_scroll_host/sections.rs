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

//! Navigation section stacking and caption affordances.

use crate::paint::rail_band::hits;
use crate::paint::rail_left_geom::{hit, nav_h, nav_sections, sections, LeftHit};
use crate::paint::rail_row::{base_name, row_at, row_band, row_h};
use crate::paint::rail_text::{RAIL_GAP, RAIL_PAD};
use crate::support::RAIL;

#[test]
fn sections_stack_and_nav_height_matches_what_they_consume() {
    for s in 0..12u32 {
        for p in 0..5u32 {
            let g = sections(RAIL_PAD, 216, 0, s, p);
            assert_eq!(g.s_list.y, g.s_head.y + g.s_head.h as i32);
            assert_eq!(g.p_head.y, g.s_list.y + (g.s_list.h + RAIL_GAP) as i32);
            assert_eq!(g.p_list.y, g.p_head.y + g.p_head.h as i32);
            assert_eq!(g.s_list.h, s * row_h());
            assert_eq!(g.p_list.h, p * row_h());
            let bottom = g.p_list.y + g.p_list.h as i32;
            assert_eq!(bottom, nav_h(s, p) as i32, "nav_h({s},{p}) tracks the last band");
        }
    }
}

#[test]
fn the_plus_affordances_sit_in_their_captions() {
    let g = nav_sections(RAIL, 0, 4, 3);
    assert!(hits(&g.s_plus, g.s_plus.x, g.s_plus.y));
    assert!(g.s_plus.x + g.s_plus.w <= g.s_head.x + g.s_head.w);
    assert!(g.p_plus.x + g.p_plus.w <= g.p_head.x + g.p_head.w);
    assert!(
        g.s_plus.y >= g.s_head.y
            && g.s_plus.y + g.s_plus.h as i32 <= g.s_head.y + g.s_head.h as i32
    );
    // A click on either affordance is that affordance, ahead of any row.
    let at = |b: crate::paint::rail_band::Band| (RAIL.x + b.x, (RAIL.y as i32 + b.y) as u32);
    let (sx, sy) = at(g.s_plus);
    assert!(matches!(hit(RAIL, 4, 3, 0, sx, sy), Some(LeftHit::NewSession)));
    let (px, py) = at(g.p_plus);
    assert!(matches!(hit(RAIL, 4, 3, 0, px, py), Some(LeftHit::AddProject)));
    let r = row_band(2, &g.p_list);
    assert_eq!(row_at(&g.p_list, 3, r.x, r.y), Some(2), "below the caption, the rows");
    let (rx, ry) = at(r);
    assert!(matches!(hit(RAIL, 4, 3, 0, rx, ry), Some(LeftHit::Project(2))));
    let (qx, qy) = at(row_band(1, &g.s_list));
    assert!(matches!(hit(RAIL, 4, 3, 0, qx, qy), Some(LeftHit::Session(1))));
    assert_eq!(base_name("/home/user/src"), "src", "a project row shows its last component");
}
