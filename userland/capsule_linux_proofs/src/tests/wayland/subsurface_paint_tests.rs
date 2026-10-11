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

//! C8u: the order a window's surfaces are painted in, and that laying the
//! ones beneath it under its frame gives the picture painting them first
//! would.

use crate::wayland_sub::blend::{lay, on, Plane};
use crate::wayland_sub::order::{layers, Stack, MAX_DEPTH};

fn stack(parent: u32, ids: &[u32]) -> Stack {
    Stack { parent, ids: ids.to_vec() }
}

#[test]
fn a_stack_splits_at_the_window_and_nests_at_each_subsurface() {
    let at = |id| {
        [(2, (5, 5)), (3, (-1, 0)), (4, (1, 1)), (5, (2, 2))]
            .into_iter()
            .find(|p| p.0 == id)
            .map(|p| p.1)
    };
    let (below, above) = layers(&[stack(1, &[2, 1, 3])], 1, &at);
    assert_eq!((below, above), (vec![(2, (5, 5))], vec![(3, (-1, 0))]));
    let nested = [stack(1, &[1, 3]), stack(3, &[4, 3, 5])];
    let (below, above) = layers(&nested, 1, &at);
    assert!(below.is_empty());
    assert_eq!(above, vec![(4, (0, 1)), (3, (-1, 0)), (5, (1, 2))]);
    assert_eq!(layers(&[], 1, &at), (vec![], vec![]), "no subsurfaces");
}

#[test]
fn a_subsurface_showing_nothing_hides_what_is_placed_on_it() {
    let at = |id| (id == 4).then_some((1, 1));
    let (below, above) = layers(&[stack(1, &[1, 3]), stack(3, &[4, 3])], 1, &at);
    assert!(below.is_empty() && above.is_empty());
}

#[test]
fn a_cycle_is_cut_at_the_deepest_placing() {
    let at = |_| Some((1, 0));
    let (_, above) = layers(&[stack(1, &[1, 2]), stack(2, &[2, 3]), stack(3, &[3, 2])], 1, &at);
    assert!(above.len() <= MAX_DEPTH + 1, "{}", above.len());
}

#[test]
fn under_and_over_build_the_same_picture_as_painting_bottom_first() {
    let p = Plane { w: 1, h: 1, stride: 4 };
    let (low, mid, top) = ([10, 200, 30, 255], [128, 0, 0, 128], [0, 40, 0, 60]);
    let window = [20, 20, 90, 100];
    let painted = on(top, on(window, on(mid, low)));
    let mut frame = window.to_vec();
    lay(&mut frame, p, &mid, p, (0, 0), false);
    lay(&mut frame, p, &low, p, (0, 0), false);
    lay(&mut frame, p, &top, p, (0, 0), true);
    for (a, b) in frame.iter().zip(painted) {
        assert!(a.abs_diff(b) <= 1, "{frame:?} against {painted:?}");
    }
}
