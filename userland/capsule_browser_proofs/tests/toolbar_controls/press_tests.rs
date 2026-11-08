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

use capsule_browser_proofs::browser::ui::toolbar_geom::Part;
use nonos_toolkit::widgets::{Controls, Step};

use super::kit::{at_scale, changed, live, paint};
use super::spots::{centre, DRAWN};

#[test]
fn each_control_lights_and_acts_only_where_it_is_drawn() {
    for (q, part) in [4, 8].into_iter().flat_map(|q| DRAWN.map(|p| (q, p))) {
        let _g = at_scale(q);
        let (s, mut c) = (live(), Controls::acting_on_press());
        let unlit = paint(&s, &mut c);
        let (x, y) = centre(part, &s);
        c.moved(x, y);
        let lit = paint(&s, &mut c);
        assert_eq!(c.pressed(x, y), Step::Act(part), "{part:?} at {q}");
        let pressed = paint(&s, &mut c);
        for diff in [changed(&unlit, &lit), changed(&unlit, &pressed)] {
            let here = diff.iter().all(|&(x, y)| c.at(x, y) == Some(part));
            assert!(!diff.is_empty() && here, "{part:?} at {q}");
        }
        c.released();
        c.left();
        assert_eq!(paint(&s, &mut c), unlit, "{part:?} once the pointer left, at {q}");
    }
}

#[test]
fn a_disabled_control_never_lights_nor_acts() {
    let (mut s, dead) = (live(), [Part::Back, Part::Fwd, Part::Star, Part::Reader]);
    (s.back, s.fwd, s.page, s.ready) = (false, false, false, false);
    for (q, part) in [4, 8].into_iter().flat_map(|q| dead.map(|p| (q, p))) {
        let _g = at_scale(q);
        let mut c = Controls::acting_on_press();
        let unlit = paint(&s, &mut c);
        let (x, y) = centre(part, &s);
        c.moved(x, y);
        assert_eq!(paint(&s, &mut c), unlit, "{part:?} lit at {q}");
        assert!(!matches!(c.pressed(x, y), Step::Act(_)), "{part:?} acted at {q}");
        assert_eq!(c.at(x, y), None, "{part:?} at {q}");
    }
}

#[test]
fn reader_stays_live_while_it_reads_with_nothing_parsed() {
    let mut s = live();
    (s.ready, s.reading) = (false, true);
    let _g = at_scale(4);
    let mut c = Controls::acting_on_press();
    paint(&s, &mut c);
    let (x, y) = centre(Part::Reader, &s);
    assert_eq!(c.pressed(x, y), Step::Act(Part::Reader));
}
