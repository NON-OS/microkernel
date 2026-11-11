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

use capsule_browser_proofs::browser::shell::route::Route;
use capsule_browser_proofs::browser::toolbar::badge;
use capsule_browser_proofs::browser::ui::toolbar_geom::Part;
use nonos_toolkit::widgets::{Controls, Step};

use super::kit::{at_scale, geom, live, paint};
use super::spots::centre;

#[test]
fn the_address_field_takes_a_press_beside_the_badge() {
    for q in [4, 5, 6, 8] {
        let _g = at_scale(q);
        let (mut s, mut c) = (live(), Controls::acting_on_press());
        s.route = Route::Direct;
        let unlit = paint(&s, &mut c);
        let body = badge::body(geom().route, s.route);
        let (left, top) = (body.x as i32, body.y as i32);
        assert_eq!(c.at(left, top), Some(Part::Route), "the badge's corner at {q}");
        assert_eq!(c.at(left - 1, top), Some(Part::Pill), "just left of it at {q}");
        let (x, y) = centre(Part::Pill, &s);
        assert_eq!(c.pressed(x, y), Step::Act(Part::Pill), "at {q}");
        assert_eq!(paint(&s, &mut c), unlit, "the field is not a widget that lights, at {q}");
    }
}
