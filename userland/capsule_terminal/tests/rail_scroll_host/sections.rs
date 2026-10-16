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
use crate::paint::rail_left_geom::{nav_h, nav_sections, sections};
use crate::paint::rail_row::row_h;
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
}
