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

//! A raise repaints where the raised window was covered and nothing else
//! changes, raising the top window or one no other covered repaints
//! nothing, and a close uncovers what was under it.

use super::repaint_rig::*;
use crate::damage::DamageAccumulator;
use crate::scene::SceneTable;
use crate::scene_raise::raise_by_pid;
use crate::scene_remove::remove_by_pid;
use crate::tests::scene::frame_model::{Screen, H, W};

#[test]
fn a_raise_repaints_where_the_raised_window_was_covered_and_nothing_else_changes() {
    let (mut scene, mut acc) = (SceneTable::new(), DamageAccumulator::new());
    open(&mut scene, &mut acc, 1, (2, 2, 30, 24));
    open(&mut scene, &mut acc, 2, (14, 10, 30, 24));
    open(&mut scene, &mut acc, 3, (26, 18, 30, 24));
    while acc.drain().is_some() {}
    let before = Screen::composed(&scene);
    let mut screen = Screen::composed(&scene);
    let rect = raise_by_pid(&mut scene, 1).expect("window 1 was covered");
    // Window 1 runs 2..32 by 2..26; 2 covered it from (14, 10), 3 from
    // (26, 18): the box round both is all that can change.
    assert_eq!((rect.x, rect.y, rect.width, rect.height), (14, 10, 18, 16));
    let after = Screen::composed(&scene);
    for y in 0..H {
        for x in 0..W {
            if before.top_at(x, y) != after.top_at(x, y) {
                let inside = (14..32).contains(&x) && (10..26).contains(&y);
                assert!(inside, "({x}, {y}) changed outside where it was covered");
            }
        }
    }
    acc.accumulate(rect);
    screen.repaint(&scene, &mut acc);
    assert_current(&screen, &scene, "after the raise");
    assert_eq!(screen.top_at(20, 20), 1, "the raised window is on top where all three overlap");
}

#[test]
fn raising_the_window_on_top_repaints_nothing() {
    let (mut scene, mut acc) = (SceneTable::new(), DamageAccumulator::new());
    open(&mut scene, &mut acc, 1, (2, 2, 30, 24));
    open(&mut scene, &mut acc, 2, (14, 10, 30, 24));
    assert!(raise_by_pid(&mut scene, 2).is_none());
    assert!(raise_by_pid(&mut scene, 99).is_none(), "an owner with no layer changes nothing");
}

/// A window stamped under another it does not meet is put on top of its
/// band, and nothing on the screen moves.
#[test]
fn raising_a_window_no_other_covers_repaints_nothing() {
    let (mut scene, mut acc) = (SceneTable::new(), DamageAccumulator::new());
    open(&mut scene, &mut acc, 1, (2, 2, 10, 10));
    open(&mut scene, &mut acc, 2, (40, 30, 10, 10));
    let before = Screen::composed(&scene);
    assert!(raise_by_pid(&mut scene, 1).is_none());
    let after = Screen::composed(&scene);
    for y in 0..H {
        for x in 0..W {
            assert_eq!(before.top_at(x, y), after.top_at(x, y), "({x}, {y})");
        }
    }
}

#[test]
fn a_closed_window_uncovers_what_was_under_it() {
    let (mut scene, mut acc) = (SceneTable::new(), DamageAccumulator::new());
    open(&mut scene, &mut acc, 1, (2, 2, 30, 24));
    open(&mut scene, &mut acc, 2, (14, 10, 30, 24));
    let mut screen = Screen::composed(&scene);
    while acc.drain().is_some() {}
    acc.accumulate(remove_by_pid(&mut scene, 2).expect("window 2 was there"));
    screen.repaint(&scene, &mut acc);
    assert_current(&screen, &scene, "after the close");
}
