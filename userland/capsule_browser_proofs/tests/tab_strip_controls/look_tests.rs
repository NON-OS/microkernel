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

use capsule_browser_proofs::browser::ui::tabs_geom::TabPart;
use nonos_toolkit::widgets::Controls;

use super::kit::{at_scale, changed, of, paint, spots, strip};

#[test]
fn choosing_a_tab_marks_only_it_and_the_tab_it_leaves() {
    for q in [4, 5, 6, 8] {
        let _g = at_scale(q);
        let (a, b, mut c) = (strip(3, 0, q), strip(3, 2, q), Controls::acting_on_press());
        let (one, two) = (paint(&a, &[false; 3], &mut c), paint(&b, &[false; 3], &mut c));
        let mine = |p: TabPart| of(0)(p) || of(2)(p);
        let diff = changed(&one, &two);
        let here = diff.iter().all(|&(x, y)| c.at(x, y).is_some_and(mine));
        assert!(!diff.is_empty() && here, "at {q}");
    }
}

#[test]
fn a_busy_tab_changes_only_its_own_box() {
    for q in [4, 5, 6, 8] {
        let _g = at_scale(q);
        let (s, mut c) = (strip(3, 0, q), Controls::acting_on_press());
        let idle = paint(&s, &[false; 3], &mut c);
        let busy = paint(&s, &[false, true, false], &mut c);
        let diff = changed(&idle, &busy);
        assert!(!diff.is_empty() && diff.iter().all(|&(x, y)| c.at(x, y).is_some_and(of(1))));
    }
}

#[test]
fn a_crowded_strip_slides_to_keep_the_active_tab_and_the_plus() {
    for (q, active) in [4, 5, 6, 8].into_iter().flat_map(|q| (0..9).map(move |a| (q, a))) {
        let _g = at_scale(q);
        let (s, mut c) = (strip(9, active, q), Controls::acting_on_press());
        paint(&s, &[false; 9], &mut c);
        assert!(!spots(&c, &s, |a| a == TabPart::Tab(active)).is_empty(), "{active} at {q}");
        assert!(!spots(&c, &s, |a| a == TabPart::Plus).is_empty(), "{active} at {q}");
        assert!((0..9).any(|i| spots(&c, &s, of(i)).is_empty()), "some tab out of view");
    }
}
