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

//! The image viewer's hover: a move inside one target repaints nothing, one onto
//! another repaints, the leave names nothing, and a press lights only its own.

use nonos_app_skeleton::InputEvent;
use nonos_app_skeleton::InputKind::{self, ButtonDown, ButtonUp, PointerAbs, Touch};
use nonos_toolkit::widgets::State as Look;

use crate::image_controls_tests::{mid, single, SIZES};
use crate::viewer::gallery::layout::{cell_rect, grid};
use crate::viewer::hover::Hover;
use crate::viewer::target::{controls, Shown, Target};

fn feed(h: &mut Hover, s: &Shown, kind: InputKind, (x, y): (i32, i32)) -> bool {
    let (flags, code, delta_x, delta_y, timestamp_ns) = (0, 0, 0, 0, 0);
    h.feed(s, &InputEvent { kind, flags, code, x, y, delta_x, delta_y, timestamp_ns })
}

#[test]
fn a_move_inside_a_control_repaints_nothing_and_one_onto_the_next_repaints() {
    for (w, h) in SIZES {
        let (s, mut hv) = (single(w, h, 3), Hover::new());
        let ((a, ra), (b, rb)) = (controls(&s).next().unwrap(), controls(&s).last().unwrap());
        assert!(feed(&mut hv, &s, PointerAbs, (ra.x as i32, ra.y as i32)));
        assert!(!feed(&mut hv, &s, PointerAbs, mid(ra)) && feed(&mut hv, &s, PointerAbs, mid(rb)));
        assert!(hv.look(&s, b).hovered && hv.look(&s, a) == Look::default());
        assert!(feed(&mut hv, &s, PointerAbs, (2, 1)) && !feed(&mut hv, &s, PointerAbs, (3, 2)));
    }
}

#[test]
fn the_leave_names_nothing_and_lights_nothing() {
    let (s, mut hv) = (single(720, 520, 2), Hover::new());
    let (next, r) = controls(&s).last().unwrap();
    assert!(feed(&mut hv, &s, PointerAbs, mid(r)) && hv.look(&s, next).hovered);
    assert!(feed(&mut hv, &s, PointerAbs, (-1, -1)) && hv.pointer.position().is_none());
    assert!(hv.over.is_none() && controls(&s).all(|(t, _)| hv.look(&s, t) == Look::default()));
    assert!(!feed(&mut hv, &s, PointerAbs, (-1, -1)));
}

#[test]
fn a_press_lights_only_its_control_and_a_touch_asks_nothing() {
    let (s, mut hv) = (single(720, 520, 2), Hover::new());
    let ((prev, r0), (next, r1)) = (controls(&s).next().unwrap(), controls(&s).last().unwrap());
    assert!(feed(&mut hv, &s, ButtonDown, mid(r0)) && hv.look(&s, prev).pressed);
    assert!(feed(&mut hv, &s, PointerAbs, mid(r1)) && !hv.look(&s, next).pressed);
    assert!(hv.look(&s, next).hovered && hv.look(&s, prev) == Look::default());
    assert!(feed(&mut hv, &s, ButtonUp, mid(r1)) && !feed(&mut hv, &s, Touch, (0, 0)));
}

#[test]
fn the_gallery_lights_one_tile_at_a_time() {
    let (s, mut hv) = (Shown { gallery: true, tiles: 3, ..single(720, 520, 0) }, Hover::new());
    for i in 0..3 {
        let ((x, y, _, _), t) = (cell_rect(i, 0, &grid(720)), Target::Tile(i));
        assert!(feed(&mut hv, &s, PointerAbs, (x, y)) && hv.over == Some(t));
        assert!(!feed(&mut hv, &s, PointerAbs, (x + 9, y + 9)) && hv.look(&s, t).hovered);
    }
    assert!(feed(&mut hv, &s, PointerAbs, (2, 2)) && hv.over.is_none());
}
