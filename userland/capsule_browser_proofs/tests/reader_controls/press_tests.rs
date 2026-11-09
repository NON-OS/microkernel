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

use capsule_browser_proofs::browser::shell::kept::{ReaderPrefs, MAX_SIZE};
use nonos_toolkit::widgets::{Controls, Step};

use super::kit::{at_scale, controls, geom, mid, paint, prefs, W};

#[test]
fn each_control_acts_at_every_point_it_is_drawn_and_nowhere_else() {
    for q in [4, 8] {
        let _g = at_scale(q);
        let (g, mut c) = (geom(), Controls::acting_on_press());
        paint(W, &prefs(), &mut c);
        let p = g.panel;
        let row = |y| (p.x - 4..p.right() + 4).step_by(3).map(move |x| (x, y));
        for (x, y) in (p.y - 4..p.bottom() + 4).step_by(3).flat_map(row) {
            let drawn = controls(&g).into_iter().find(|(_, r)| r.contains(x, y));
            assert_eq!(c.at(x, y), drawn.map(|(k, _)| k), "({x}, {y}) at {q}");
        }
        for (ctl, r) in controls(&g) {
            let (x, y) = mid(r);
            assert_eq!(c.pressed(x, y), Step::Act(ctl), "{ctl:?} at {q}");
            c.released();
        }
    }
}

#[test]
fn the_text_size_steps_are_dead_at_either_end() {
    for q in [4, 8] {
        let _g = at_scale(q);
        let g = geom();
        for (size, dead, live) in [(0, g.minus, g.plus), (MAX_SIZE, g.plus, g.minus)] {
            let mut c = Controls::acting_on_press();
            paint(W, &ReaderPrefs { size, ..prefs() }, &mut c);
            let (x, y) = mid(dead);
            assert_eq!(c.pressed(x, y), Step::Idle, "size {size} at {q}");
            c.released();
            let (x, y) = mid(live);
            assert!(matches!(c.pressed(x, y), Step::Act(_)), "size {size} at {q}");
        }
    }
}
