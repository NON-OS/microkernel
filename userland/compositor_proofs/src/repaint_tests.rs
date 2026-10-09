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

//! Moving, raising, opening and closing windows repaints exactly enough. The
//! screen is kept current only through the damage the real submit, raise and
//! remove steps report, drained through the real accumulator, and after every
//! step it must equal the scene composed from scratch. A pixel that differs is
//! a half-drawn region on the real screen: a copy of a moved window left
//! behind, or part of a raised window still under the one it came up over.

use crate::damage::{DamageAccumulator, Rect};
use crate::frame_model::{layer, Rng, Screen, BACKGROUND, H, W};
use crate::scene::{Layer, SceneTable};
use crate::scene_raise::raise_by_pid;
use crate::scene_remove::remove_by_pid;
use crate::scene_submit::{rect_of, submit_layer};

fn open(scene: &mut SceneTable, acc: &mut DamageAccumulator, owner: u32, r: (u32, u32, u32, u32)) {
    let l = layer(owner, r.0, r.1, r.2, r.3, 2);
    for rect in submit_layer(scene, l).expect("room for the layer").into_iter().flatten() {
        acc.accumulate(rect);
    }
}

fn assert_current(screen: &Screen, scene: &SceneTable, what: &str) {
    let full = Screen::composed(scene);
    if let Some((x, y, shown, wanted)) = screen.first_difference(&full) {
        panic!("{what}: pixel ({x}, {y}) shows owner {shown}, a full frame shows {wanted}");
    }
}

#[test]
fn a_moved_window_leaves_no_copy_behind() {
    let (mut scene, mut acc) = (SceneTable::new(), DamageAccumulator::new());
    open(&mut scene, &mut acc, 1, (4, 4, 20, 16));
    open(&mut scene, &mut acc, 2, (34, 8, 20, 24));
    let mut screen = Screen::composed(&scene);
    while acc.drain().is_some() {}
    // The client drags window 1 down and right, the way app_skeleton submits
    // each step of a title bar drag: one scene submit at the new place.
    open(&mut scene, &mut acc, 1, (12, 22, 20, 16));
    screen.repaint(&scene, &mut acc);
    assert_current(&screen, &scene, "after the move");
    assert_eq!(screen.top_at(4, 4), 0, "the old top-left corner shows the desktop again");
}

#[test]
fn a_move_reports_both_the_old_and_the_new_place() {
    let mut scene = SceneTable::new();
    let _ = submit_layer(&mut scene, layer(7, 10, 10, 8, 8, 2));
    let out = submit_layer(&mut scene, layer(7, 30, 20, 8, 8, 2)).expect("resubmit");
    let rects: Vec<_> = out.iter().flatten().map(|r| (r.x, r.y, r.width, r.height)).collect();
    assert!(rects.contains(&(30, 20, 8, 8)), "new place: {rects:?}");
    assert!(rects.contains(&(10, 10, 8, 8)), "old place: {rects:?}");
}

#[test]
fn a_resubmit_in_place_reports_one_rectangle() {
    let mut scene = SceneTable::new();
    let _ = submit_layer(&mut scene, layer(7, 10, 10, 8, 8, 2));
    let out = submit_layer(&mut scene, layer(7, 10, 10, 8, 8, 2)).expect("resubmit");
    assert_eq!(out.iter().flatten().count(), 1);
}

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

/// Long random sessions: windows open in three bands, move, resize, get
/// raised and close, and the screen is only ever repainted where the steps
/// said. It must match a full frame after every single step.
#[test]
fn random_sessions_never_leave_a_stale_pixel() {
    for seed in 1..=96u64 {
        let mut rng = Rng::new(seed);
        let (mut scene, mut acc) = (SceneTable::new(), DamageAccumulator::new());
        let mut screen = Screen::composed(&scene);
        let mut live: Vec<u32> = Vec::new();
        let mut next_owner = 1u32;
        for step in 0..400 {
            let what = rng.below(10);
            if live.is_empty() || (what < 2 && live.len() < 8) {
                let (w, h) = (4 + rng.below(W / 2), 4 + rng.below(H / 2));
                let (x, y) = (rng.below(W - w), rng.below(H - h));
                let band = [1, 2, 2, 2, 3][rng.below(5) as usize];
                let l = layer(next_owner, x, y, w, h, band);
                for r in submit_layer(&mut scene, l).expect("room").into_iter().flatten() {
                    acc.accumulate(r);
                }
                live.push(next_owner);
                next_owner += 1;
            } else if what < 6 {
                let owner = live[rng.below(live.len() as u32) as usize];
                let old = scene.layers().find(|l| l.owner_pid == owner).copied().expect("live");
                let (w, h) = if rng.below(4) == 0 {
                    (4 + rng.below(W / 2), 4 + rng.below(H / 2))
                } else {
                    (old.width, old.height)
                };
                let (x, y) = (rng.below(W - w), rng.below(H - h));
                let l = layer(owner, x, y, w, h, old.z);
                for r in submit_layer(&mut scene, l).expect("resubmit").into_iter().flatten() {
                    acc.accumulate(r);
                }
            } else if what < 9 {
                let owner = live[rng.below(live.len() as u32) as usize];
                if let Some(r) = raise_by_pid(&mut scene, owner) {
                    acc.accumulate(r);
                }
            } else {
                let i = rng.below(live.len() as u32) as usize;
                let owner = live.swap_remove(i);
                if let Some(r) = remove_by_pid(&mut scene, owner) {
                    acc.accumulate(r);
                }
            }
            screen.repaint(&scene, &mut acc);
            assert_current(&screen, &scene, &format!("seed {seed}, step {step}"));
        }
    }
}

/// What composite::paint does for one damaged rectangle, with the surfaces in
/// `dead` failing to attach: compose without them, then reap, repainting the
/// rectangle of every layer the reaper drops.
fn composite(
    screen: &mut Screen,
    scene: &mut SceneTable,
    acc: &mut DamageAccumulator,
    clip: Rect,
    dead: &[u64],
) {
    screen.paint_except(scene, clip, dead);
    let attached: Vec<u64> =
        scene.layers().map(|l| l.surface_handle).filter(|h| !dead.contains(h)).collect();
    let mut dropped = [Layer::default(); 32];
    let n = scene.reap_unattachable(&attached, REAP_THRESHOLD, &mut dropped);
    for l in &dropped[..n] {
        acc.accumulate(rect_of(l));
    }
}

const REAP_THRESHOLD: u16 = 60;

/// A window whose owner died without removing it: its surface stops
/// attaching, and once the reaper drops it the screen must not keep showing
/// it. The reaper used to hand back only the surface handle, so nothing
/// repainted where the window had been until the next periodic full frame.
#[test]
fn a_reaped_window_does_not_stay_on_screen() {
    let (mut scene, mut acc) = (SceneTable::new(), DamageAccumulator::new());
    open(&mut scene, &mut acc, 1, (2, 2, 40, 30));
    open(&mut scene, &mut acc, 2, (20, 14, 40, 30));
    let mut screen = Screen::composed(&scene);
    while acc.drain().is_some() {}
    assert_eq!(screen.top_at(30, 20), 2);
    // Owner 2 dies. The cursor keeps the compositor painting a small
    // rectangle far from it, frame after frame.
    let dead = [2u64];
    for _ in 0..(REAP_THRESHOLD as usize + 5) {
        acc.accumulate(Rect { x: 0, y: 44, width: 4, height: 4 });
        while let Some(r) = acc.drain() {
            composite(&mut screen, &mut scene, &mut acc, r, &dead);
        }
    }
    assert!(scene.layers().all(|l| l.owner_pid != 2), "the reaper dropped the dead layer");
    assert_current(&screen, &scene, "after the reap");
    assert_eq!(screen.top_at(30, 20), 1, "the window under it shows again");
}

#[test]
fn the_reaper_hands_back_the_whole_layer() {
    let mut scene = SceneTable::new();
    let _ = submit_layer(&mut scene, layer(9, 3, 4, 5, 6, 2));
    let mut dropped = [Layer::default(); 4];
    for _ in 0..2 {
        assert_eq!(scene.reap_unattachable(&[], 3, &mut dropped), 0);
    }
    assert_eq!(scene.reap_unattachable(&[], 3, &mut dropped), 1);
    let d = dropped[0];
    assert_eq!((d.owner_pid, d.surface_handle, d.x, d.y, d.width, d.height), (9, 9, 3, 4, 5, 6));
}

#[test]
fn a_damaged_rectangle_never_reaches_past_the_layer_it_names() {
    let mut scene = SceneTable::new();
    let _ = submit_layer(&mut scene, layer(1, 0, 0, 10, 10, 2));
    let _ = submit_layer(&mut scene, layer(2, 5, 5, 10, 10, 2));
    let r: Rect = raise_by_pid(&mut scene, 1).expect("covered");
    assert_eq!((r.x, r.y, r.width, r.height), (0, 0, 10, 10));
}

/// A window on a fresh surface: `handle` is the new surface, drawn or not.
fn resized(owner: u32, handle: u64, r: (u32, u32, u32, u32)) -> Layer {
    Layer { surface_handle: handle, ..layer(owner, r.0, r.1, r.2, r.3, 2) }
}

fn submit(scene: &mut SceneTable, acc: &mut DamageAccumulator, l: Layer) {
    for rect in submit_layer(scene, l).expect("room for the layer").into_iter().flatten() {
        acc.accumulate(rect);
    }
}

/// Compose every damaged rectangle with the surfaces in `blank` drawn as
/// what they hold: nothing yet, so whatever is under them shows.
fn frame_with_blank(
    screen: &mut Screen,
    scene: &SceneTable,
    acc: &mut DamageAccumulator,
    blank: &[u64],
) {
    while let Some(r) = acc.drain() {
        screen.paint_except(scene, r, blank);
    }
}

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
