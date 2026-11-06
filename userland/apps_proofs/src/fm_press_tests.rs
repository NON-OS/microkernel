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

//! The file manager's leave and press: the router's leave names nothing and
//! lights nothing, a press lights only the target it began on, and a touch
//! asks for no repaint.

use nonos_app_skeleton::InputKind::{ButtonDown, ButtonUp, PointerAbs, Touch};
use nonos_toolkit::widgets::State as Look;

use crate::fm::hover::Hover;
use crate::fm_check::points;
use crate::fm_controls_tests::{drawn, pane};
use crate::fm_hover_tests::{at, feed, LEAVE};

#[test]
fn the_leave_names_nothing_and_lights_nothing() {
    let (p, mut hv) = (pane(1100, 700, false, 3), Hover::new());
    let (t, r) = drawn(&p)[6];
    assert!(feed(&mut hv, &p, PointerAbs, points(r)[0]) && hv.over == Some(t));
    assert!(feed(&mut hv, &p, PointerAbs, LEAVE) && hv.pointer.position().is_none());
    assert!(drawn(&p).iter().all(|&(t, _)| hv.look(t, |x, y| at(&p, x, y)) == Look::default()));
    assert!(hv.over.is_none() && !feed(&mut hv, &p, PointerAbs, LEAVE));
}

#[test]
fn a_press_lights_only_its_target_and_a_touch_asks_nothing() {
    let (p, mut hv) = (pane(1100, 700, false, 0), Hover::new());
    let cs = drawn(&p);
    let ((a, ra), (b, rb)) = (cs[5], cs[7]);
    let look = |hv: &Hover, t| hv.look(t, |x, y| at(&p, x, y));
    assert!(feed(&mut hv, &p, ButtonDown, points(ra)[0]) && look(&hv, a).pressed);
    assert!(feed(&mut hv, &p, PointerAbs, points(rb)[0]) && !look(&hv, b).pressed);
    assert!(look(&hv, b).hovered && look(&hv, a) == Look::default());
    assert!(feed(&mut hv, &p, ButtonUp, points(rb)[0]) && !feed(&mut hv, &p, Touch, (0, 0)));
}
