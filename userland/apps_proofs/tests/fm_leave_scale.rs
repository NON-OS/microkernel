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

//! The file manager's leave at 1x, 1.25x, 1.5x and 2x in a window grown by the
//! same scale: the router's leave names nothing and lights nothing. Its own
//! binary, so setting the scale races no other test.

use apps_proofs::fm::hover::Hover;
use apps_proofs::fm_check::{at, feed, points, LEAVE};
use apps_proofs::fm_window::{dims, drawn, pane};
use nonos_app_skeleton::InputKind::PointerAbs;
use nonos_toolkit::design::scale::set_scale;
use nonos_toolkit::widgets::State as Look;

#[test]
fn the_leave_names_nothing_and_lights_nothing_at_every_scale() {
    for q in [4, 5, 6, 8] {
        set_scale(q);
        let (w, h) = dims(1100, 700);
        let (p, mut hv) = (pane(w, h, false, 3), Hover::new());
        let (t, r) = drawn(&p)[6];
        assert!(feed(&mut hv, &p, PointerAbs, points(r)[0]) && hv.over == Some(t), "q={q}");
        assert!(feed(&mut hv, &p, PointerAbs, LEAVE) && hv.pointer.position().is_none());
        let dark =
            drawn(&p).iter().all(|&(t, _)| hv.look(t, |x, y| at(&p, x, y)) == Look::default());
        assert!(dark, "q={q}");
        assert!(hv.over.is_none() && !feed(&mut hv, &p, PointerAbs, LEAVE), "q={q}");
    }
    set_scale(4);
}
