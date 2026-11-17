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

//! The file manager's press at 1x, 1.25x, 1.5x and 2x in a window grown by the
//! same scale: a press lights only the target it began on, and a touch asks
//! for no repaint. Its own binary, so setting the scale races no other test.

use apps_proofs::fm::hover::Hover;
use apps_proofs::fm_check::{at, feed, points};
use apps_proofs::fm_window::{dims, drawn, pane};
use nonos_app_skeleton::InputKind::{ButtonDown, ButtonUp, PointerAbs, Touch};
use nonos_toolkit::design::scale::set_scale;
use nonos_toolkit::widgets::State as Look;

#[test]
fn a_press_lights_only_its_target_and_a_touch_asks_nothing_at_every_scale() {
    for q in [4, 5, 6, 8] {
        set_scale(q);
        let (w, h) = dims(1100, 700);
        let (p, mut hv) = (pane(w, h, false, 0), Hover::new());
        let cs = drawn(&p);
        let ((a, ra), (b, rb)) = (cs[5], cs[7]);
        let look = |hv: &Hover, t| hv.look(t, |x, y| at(&p, x, y));
        assert!(feed(&mut hv, &p, ButtonDown, points(ra)[0]) && look(&hv, a).pressed, "q={q}");
        assert!(feed(&mut hv, &p, PointerAbs, points(rb)[0]) && !look(&hv, b).pressed, "q={q}");
        assert!(look(&hv, b).hovered && look(&hv, a) == Look::default(), "q={q}");
        assert!(feed(&mut hv, &p, ButtonUp, points(rb)[0]), "q={q}");
        assert!(!feed(&mut hv, &p, Touch, (0, 0)), "q={q}");
    }
    set_scale(4);
}
