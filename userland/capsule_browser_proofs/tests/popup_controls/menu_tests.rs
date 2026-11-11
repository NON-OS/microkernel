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

use capsule_browser_proofs::browser::ui::select_menu::row_at;

use super::kit::{at_scale, changed, list_px, open, PAGE, SIZES};
use super::spots::{select, spots};

#[test]
fn an_option_lights_and_is_chosen_only_where_it_is_drawn() {
    for q in [4, 5, 6, 8] {
        let _g = at_scale(q);
        let (mut list, at) = open(SIZES, select(), PAGE);
        list.hi = 3;
        let (unlit, all) = (list_px(&list, at), spots(&list, at));
        for i in [0, 1, 4, 5] {
            list.hi = i;
            let lit = changed(&unlit, &list_px(&list, at));
            assert!(!lit.is_empty(), "row {i} lights at {q}");
            assert!(lit.iter().all(|&(x, y)| row_at(&list, at, x, y) == Some(i)), "{i} at {q}");
            let (y0, y1) = (lit.iter().map(|p| p.1).min(), lit.iter().map(|p| p.1).max());
            let mut hits = all.iter().filter(|(_, r)| *r == Some(i)).map(|(p, _)| p.1);
            assert!(hits.clone().count() > 0, "row {i} is chosen at {q}");
            assert!(hits.all(|y| Some(y) >= y0 && Some(y) <= y1), "row {i} only where lit at {q}");
            assert_eq!(list.highlighted(), Some(list.rows[i].id));
        }
    }
}

#[test]
fn the_current_option_is_ticked() {
    for q in [4, 5, 6, 8] {
        let _g = at_scale(q);
        let (mut list, at) = open(SIZES, select(), PAGE);
        list.hi = 3;
        let on = list_px(&list, at);
        list.rows[1].selected = false;
        let tick = changed(&on, &list_px(&list, at));
        assert!(!tick.is_empty(), "Medium is ticked at {q}");
        assert!(tick.iter().all(|&(x, y)| row_at(&list, at, x, y) == Some(1)), "at {q}");
    }
}
