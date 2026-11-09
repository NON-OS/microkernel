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

use capsule_browser_proofs::browser::sidebar::nav::Look;
use capsule_browser_proofs::browser::ui::sidebar_geom::{rows, RowKind};
use nonos_toolkit::widgets::{Controls, Step};

use super::kit::{at_scale, changed, look, mid, paint, side};

pub fn scales() -> impl Iterator<Item = (u32, bool)> {
    [4, 8].into_iter().flat_map(|q| [(q, false), (q, true)])
}

#[test]
fn each_row_lights_and_acts_only_where_it_is_drawn() {
    for (q, rail) in scales() {
        let _g = at_scale(q);
        let (s, k) = (side(rail, 1400), look(Some(RowKind::Tabs), false));
        for row in rows(rail, s, k.groups.len()) {
            let (mut c, (x, y)) = (Controls::acting_on_press(), mid(row.rect));
            let unlit = paint(rail, s, &k, &mut c);
            c.moved(x, y);
            let lit = paint(rail, s, &k, &mut c);
            assert_eq!(c.pressed(x, y), Step::Act(row.kind), "{row:?} {rail} at {q}");
            let pressed = paint(rail, s, &k, &mut c);
            for diff in [changed(&unlit, &lit, s), changed(&unlit, &pressed, s)] {
                assert!(diff.iter().all(|&(x, y)| c.at(x, y) == Some(row.kind)), "{row:?} {q}");
                assert!(row.kind == RowKind::Tabs || !diff.is_empty(), "{row:?} {rail} {q}");
            }
            c.released();
            c.left();
            assert_eq!(paint(rail, s, &k, &mut c), unlit, "{row:?} once left, at {q}");
        }
    }
}

#[test]
fn a_press_lands_on_the_row_drawn_at_every_centre() {
    let named = [("g", 1); 8];
    for (q, rail) in scales() {
        let _g = at_scale(q);
        for (h, g) in (560..=1400).step_by(21).flat_map(|h| [0, 3, 8].map(|g| (h, g))) {
            let (s, mut c) = (side(rail, h), Controls::acting_on_press());
            let k = Look { on: None, tabs: 3, groups: &named[..g], full: false };
            paint(rail, s, &k, &mut c);
            for row in rows(rail, s, g) {
                let (x, y) = mid(row.rect);
                assert_eq!(c.at(x, y), Some(row.kind), "{rail} {s:?} {g} at {q}");
            }
            assert_eq!((c.at(s.right(), s.y + 100), c.at(s.x + 1, s.y - 1)), (None, None));
        }
    }
}
