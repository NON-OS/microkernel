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

use capsule_browser_proofs::browser::ui::tabs_geom::TabPart::{self, Close, Plus, Tab};
use nonos_toolkit::widgets::{Controls, Step};

use super::kit::{at_scale, centre, changed, of, paint, spots, strip, W};

const PARTS: [TabPart; 6] = [Tab(1), Tab(2), Close(0), Close(1), Close(2), Plus];

fn owner(part: TabPart) -> Box<dyn Fn(TabPart) -> bool> {
    match part {
        TabPart::Tab(i) | TabPart::Close(i) => Box::new(of(i)),
        TabPart::Plus => Box::new(|a| a == TabPart::Plus),
    }
}

#[test]
fn each_control_lights_and_acts_only_where_it_is_drawn() {
    for (q, part) in [4, 8].into_iter().flat_map(|q| PARTS.map(|p| (q, p))) {
        let _g = at_scale(q);
        let (s, mut c) = (strip(3, 0, q), Controls::acting_on_press());
        let unlit = paint(&s, &[false; 3], &mut c);
        let (x, y) = centre(&c, &s, part);
        c.moved(x, y);
        let lit = paint(&s, &[false; 3], &mut c);
        assert_eq!(c.pressed(x, y), Step::Act(part), "{part:?} at {q}");
        let pressed = paint(&s, &[false; 3], &mut c);
        let mine = owner(part);
        for diff in [changed(&unlit, &lit), changed(&unlit, &pressed)] {
            let here = diff.iter().all(|&(x, y)| c.at(x, y).is_some_and(&mine));
            assert!(!diff.is_empty() && here, "{part:?} at {q}");
        }
        c.released();
        c.left();
        assert_eq!(paint(&s, &[false; 3], &mut c), unlit, "{part:?} once the pointer left, at {q}");
    }
}

#[test]
fn a_close_mark_lies_on_its_own_tab_and_closes_it() {
    for q in [4, 8] {
        let _g = at_scale(q);
        let (s, mut c) = (strip(3, 1, q), Controls::acting_on_press());
        paint(&s, &[false; 3], &mut c);
        for i in 0..3 {
            let (mark, tab) = (spots(&c, &s, |a| a == Close(i)), spots(&c, &s, of(i)));
            assert!(!mark.is_empty() && mark.iter().all(|p| tab.contains(p)), "{i} at {q}");
            let (x, y) = centre(&c, &s, TabPart::Close(i));
            assert_eq!(c.pressed(x, y), Step::Act(TabPart::Close(i)), "{i} at {q}");
            c.released();
        }
        let (x, y) = centre(&c, &s, TabPart::Tab(2));
        assert_eq!(c.pressed(x, y), Step::Act(TabPart::Tab(2)), "the tab's own body at {q}");
        assert_eq!((c.at(W - 1, s.h / 2), c.at(10, 0), c.at(-1, s.h / 2)), (None, None, None));
    }
}
