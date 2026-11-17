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

//! The video player's controls at 1x, 1.25x, 1.5x and 2x in a window grown by
//! the same scale: each is named where it is drawn, at its middle and corners,
//! and no two overlap. Its own binary, so setting the scale races no other
//! test.

use apps_proofs::ui::target::target_at;
use apps_proofs::ui::targets::targets;
use apps_proofs::video_check::{meets, points, windows};
use nonos_toolkit::design::scale::set_scale;

fn every_control_is_named_at_its_middle_and_its_corners(q: u32) {
    for s in windows() {
        for (t, r) in targets(&s).into_iter().filter(|(_, r)| r.w > 0 && r.h > 0) {
            for (x, y) in points(r) {
                assert_eq!(target_at(&s, x, y), Some(t), "q={q}: {t:?} at {x},{y} in {s:?}");
            }
        }
    }
}

fn no_two_controls_overlap(q: u32) {
    for s in windows() {
        let ts = targets(&s);
        for (i, &(a, ra)) in ts.iter().enumerate() {
            assert!(ts[i + 1..].iter().all(|&(_, rb)| !meets(ra, rb)), "q={q}: {a:?} in {s:?}");
        }
    }
}

#[test]
fn every_control_answers_where_it_is_drawn_at_every_scale() {
    for q in [4, 5, 6, 8] {
        set_scale(q);
        every_control_is_named_at_its_middle_and_its_corners(q);
        no_two_controls_overlap(q);
    }
    set_scale(4);
}
