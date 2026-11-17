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

//! About's sidebar answers a click where it draws: at 1x, 1.25x, 1.5x and 2x
//! every point of an entry's drawn box names that entry, the rows between
//! two entries name none, and the band the hit test answers for an entry is
//! the box's own rows across the sidebar. Its own binary, so setting the
//! scale races no other test.

use apps_proofs::about::section::SECTIONS;
use apps_proofs::about::ui::hit::at;
use apps_proofs::about::ui::metrics::metrics;
use apps_proofs::about::ui::nav_geom::rect;
use nonos_toolkit::design::scale::set_scale;

#[test]
fn every_drawn_entry_is_hit_where_it_is_drawn_at_every_scale() {
    for q in [4, 5, 6, 8] {
        set_scale(q);
        let side = metrics().sidebar_w as i32;
        for (i, s) in SECTIONS.iter().enumerate() {
            let r = rect(i);
            let (x0, y0) = (r.x as i32, r.y as i32);
            let (x1, y1) = (x0 + r.w as i32, y0 + r.h as i32);
            assert!(0 <= x0 && x1 <= side, "q={q}: entry {i} leaves the band");
            for y in y0..y1 {
                assert!((x0..x1).all(|x| at(x, y) == Some(*s)), "q={q}: entry {i} row {y}");
                assert!(at(0, y) == Some(*s) && at(side - 1, y) == Some(*s), "q={q}: {i} {y}");
                assert!(at(side, y).is_none() && at(-1, y).is_none(), "q={q}: {i} {y} outside");
            }
            assert!(at(x0, y0 - 1) != Some(*s) && at(x0, y1) != Some(*s), "q={q}: {i} edge");
            let next = if i + 1 < SECTIONS.len() { rect(i + 1).y as i32 } else { y1 + 1 };
            assert!((y1..next).all(|y| at(x0, y).is_none()), "q={q}: between {i} and the next");
        }
    }
    set_scale(4);
}
