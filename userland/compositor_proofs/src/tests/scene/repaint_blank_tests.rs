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

//! A window moved onto a freshly painted surface is on every frame, where
//! taking it out first used to show the desktop through it.

use super::repaint_frame::*;
use super::repaint_rig::*;
use crate::damage::DamageAccumulator;
use crate::scene::SceneTable;
use crate::scene_remove::remove_by_pid;
use crate::tests::scene::frame_model::{Screen, BACKGROUND};

/// A maximize, a restore or a resize step puts the window on a new surface.
/// app_skeleton paints the new surface first and submits it in the old
/// layer's place: the compositor replaces the layer in its band and damages
/// the old and new rectangles, so the frame after the submit already shows
/// the window whole at its new size and nothing of it at the old one.
#[test]
fn a_window_moved_onto_a_painted_surface_is_on_every_frame() {
    let (mut scene, mut acc) = (SceneTable::new(), DamageAccumulator::new());
    open(&mut scene, &mut acc, 1, (4, 4, 20, 16));
    open(&mut scene, &mut acc, 2, (40, 30, 10, 10));
    let mut screen = Screen::composed(&scene);
    while acc.drain().is_some() {}
    // Maximize: painted, then submitted.
    submit(&mut scene, &mut acc, resized(1, 101, (0, 2, 60, 40)));
    screen.repaint(&scene, &mut acc);
    assert_current(&screen, &scene, "after the maximize");
    assert_eq!(scene.layers().filter(|l| l.owner_pid == 1).count(), 1, "one layer, replaced");
    assert_eq!(screen.top_at(10, 10), 1);
    // Restore: back to a smaller rectangle; the part it left shows the
    // desktop and the window it uncovered.
    submit(&mut scene, &mut acc, resized(1, 102, (4, 4, 20, 16)));
    screen.repaint(&scene, &mut acc);
    assert_current(&screen, &scene, "after the restore");
    assert_eq!(screen.top_at(10, 10), 1);
    assert_eq!(screen.top_at(45, 35), 2, "the window under the maximized one shows again");
}

/// The order this replaced: the window taken out of the scene, then its new
/// surface submitted before it was painted. A frame can land after either
/// step, and both show the desktop where the window is.
#[test]
fn taking_the_window_out_first_showed_the_desktop_through_it() {
    let (mut scene, mut acc) = (SceneTable::new(), DamageAccumulator::new());
    open(&mut scene, &mut acc, 1, (4, 4, 20, 16));
    let mut screen = Screen::composed(&scene);
    while acc.drain().is_some() {}
    acc.accumulate(remove_by_pid(&mut scene, 1).expect("the window was there"));
    frame_with_blank(&mut screen, &scene, &mut acc, &[]);
    assert_eq!(screen.top_at(10, 10), BACKGROUND, "a frame after the remove: no window");
    submit(&mut scene, &mut acc, resized(1, 101, (0, 2, 60, 40)));
    frame_with_blank(&mut screen, &scene, &mut acc, &[101]);
    assert_eq!(screen.top_at(10, 10), BACKGROUND, "a frame after the blank submit: no window");
}
