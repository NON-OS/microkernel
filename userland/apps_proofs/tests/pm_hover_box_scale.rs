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

//! The input router's leave names nothing, and the hover moves nothing: what
//! every point of the window names is the same whatever is hovered, at 1x,
//! 1.25x, 1.5x and 2x in a window grown by the same scale. The points are
//! twelve logical pixels apart at every scale. Its own binary, so setting the
//! scale races no other test.

use apps_proofs::pm::state::{State, SCREENS};
use apps_proofs::pm::ui::hit::Target;
use apps_proofs::pm::ui::hover::target_at;
use apps_proofs::pm_window::{at, size, window};
use nonos_toolkit::design::scale::{scale, set_scale};

fn the_input_routers_leave_lands_on_nothing(q: u32) {
    for s in SCREENS {
        let state = window(s, 0);
        let left = target_at(&state, -1, -1).is_none() && target_at(&state, -30, -45).is_none();
        assert!(left, "q={q}: the leave names a target");
    }
}

/// What the window names at points twelve logical pixels apart.
fn map(state: &State) -> Vec<Option<Target>> {
    let ((w, h), step) = (size(), scale().px(12) as usize);
    let points = (0..h).step_by(step).flat_map(|y| (0..w).step_by(step).map(move |x| (x, y)));
    points.map(|(x, y)| at(state, x, y)).collect()
}

fn hover_never_moves_a_box(q: u32) {
    for s in SCREENS {
        let mut state = window(s, 2);
        let before = map(&state);
        let mut seen = Vec::new();
        for t in &before {
            if !seen.contains(t) {
                seen.push(*t);
            }
        }
        assert!(seen.len() > SCREENS.len(), "q={q}");
        for t in seen {
            state.hover = t;
            assert!(map(&state) == before, "q={q}");
        }
    }
}

/// Both box proofs at each scale in turn, in one test so no two set the scale at once.
#[test]
fn boxes_at_every_scale() {
    for q in [4, 5, 6, 8] {
        set_scale(q);
        the_input_routers_leave_lands_on_nothing(q);
        hover_never_moves_a_box(q);
    }
    set_scale(4);
}
