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
//! The splash's canvas following: failed answers, the redraw cap, and the
//! stay carried across a redraw.

use crate::follow::{Follow, MAX_REDRAWS};

const FW: (u32, u32, u32) = (1280, 800, 5120);
const GPU: (u32, u32, u32) = (1920, 1080, 7680);

#[test]
fn failed_answers_change_nothing() {
    let mut f = Follow::new(FW, 0);
    for e in [-11, -1, -22] {
        assert_eq!(f.seen(250, Err(e)), None);
    }
    assert_eq!((f.size, f.redraws), (FW, 0));
}

#[test]
fn a_canvas_that_keeps_moving_is_left_after_four_redraws() {
    assert_eq!(MAX_REDRAWS, 4);
    let mut f = Follow::new(FW, 0);
    for i in 0..4u32 {
        let s = if i % 2 == 0 { GPU } else { FW };
        assert_eq!(f.seen(250 * (i as i64 + 1), Ok(s)), Some(s));
    }
    assert!(!f.due(10_000));
    assert_eq!(f.seen(10_000, Ok((800, 600, 3200))), None);
    assert_eq!(f.redraws, 4);
}

#[test]
fn the_stay_is_kept_across_a_redraw() {
    // start and desktop_up_at belong to the Follow, not to one drawing.
    let mut f = Follow::new(FW, 3527);
    f.desktop_up_at = 9000;
    f.iters = 77;
    let _ = f.seen(4000, Ok(GPU));
    assert_eq!((f.start, f.desktop_up_at, f.iters), (3527, 9000, 77));
}
