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

//! A raise repaints the raised window and nothing else changes, raising the
//! top window repaints nothing, and a close uncovers what was under it.

use super::repaint_rig::*;
use crate::damage::DamageAccumulator;
use crate::scene::SceneTable;
use crate::scene_raise::raise_by_pid;
use crate::scene_remove::remove_by_pid;
use crate::tests::scene::frame_model::{Screen, H, W};

#[test]
fn a_raise_repaints_the_raised_window_and_nothing_else_changes() {
    let (mut scene, mut acc) = (SceneTable::new(), DamageAccumulator::new());
    open(&mut scene, &mut acc, 1, (2, 2, 30, 24));
    open(&mut scene, &mut acc, 2, (14, 10, 30, 24));
    open(&mut scene, &mut acc, 3, (26, 18, 30, 24));
    while acc.drain().is_some() {}
    let before = Screen::composed(&scene);
    let mut screen = Screen::composed(&scene);
    let rect = raise_by_pid(&mut scene, 1).expect("window 1 was covered");
    assert_eq!((rect.x, rect.y, rect.width, rect.height), (2, 2, 30, 24));
    let after = Screen::composed(&scene);
    // Every pixel the raise changes lies inside the raised window.
    for y in 0..H {
        for x in 0..W {
            if before.top_at(x, y) != after.top_at(x, y) {
                let inside = (2..32).contains(&x) && (2..26).contains(&y);
                assert!(inside, "({x}, {y}) changed outside the raised window");
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
