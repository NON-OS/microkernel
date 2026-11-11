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

//! The file manager's hover at 1x, 1.25x, 1.5x and 2x in a window grown by
//! the same scale, fed as `app.rs` feeds it: a move inside one target repaints
//! nothing and one onto another repaints. Its own binary, so setting the scale
//! races no other test.

use apps_proofs::fm::hover::Hover;
use apps_proofs::fm_check::{at, feed, points};
use apps_proofs::fm_window::{dims, drawn, pane};
use nonos_app_skeleton::InputKind::PointerAbs;
use nonos_toolkit::design::scale::set_scale;

#[test]
fn a_move_inside_a_target_repaints_nothing_and_one_onto_the_next_repaints_at_every_scale() {
    for q in [4, 5, 6, 8] {
        set_scale(q);
        let (w, h) = dims(1100, 700);
        for (grid, scroll) in [(false, 0), (false, 9), (true, 0), (true, 11)] {
            let (p, mut hv) = (pane(w, h, grid, scroll), Hover::new());
            for &(t, r) in drawn(&p).iter().rev().take(6) {
                let [mid, a, b, c, d] = points(r);
                assert!(feed(&mut hv, &p, PointerAbs, a), "q={q}: onto {t:?}");
                let still = [mid, b, c, d].iter().all(|&o| !feed(&mut hv, &p, PointerAbs, o));
                assert!(still, "q={q}: in {t:?}");
                assert!(hv.over == Some(t) && hv.look(t, |x, y| at(&p, x, y)).hovered, "q={q}");
            }
        }
    }
    set_scale(4);
}
