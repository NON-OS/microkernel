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

//! The draw order is the window manager's stack. The window manager gives
//! each raised window a new, higher z and its hit test picks the highest z
//! under the pointer; the compositor is told of every raise. These pin that
//! the compositor then draws, at every point, the window the most recent raise
//! put there, within the layer's band, so what is visibly on top is what the
//! next click lands on.

use crate::frame_model::{layer, Rng, Screen, H, W};
use crate::scene::SceneTable;
use crate::scene_raise::raise_by_pid;
use crate::scene_submit::submit_layer;

/// A window as the window manager would hold it: owner, rect, z.
type WmEntry = (u32, (u32, u32, u32, u32), u32);

fn open(scene: &mut SceneTable, owner: u32, r: (u32, u32, u32, u32), band: u32) {
    submit_layer(scene, layer(owner, r.0, r.1, r.2, r.3, band)).expect("room for the layer");
}

fn order(scene: &SceneTable) -> Vec<u32> {
    let (layers, n) = scene.z_sorted_snapshot();
    layers[..n].iter().map(|l| l.owner_pid).collect()
}

#[test]
fn a_new_window_opens_on_top_of_its_band() {
    let mut scene = SceneTable::new();
    open(&mut scene, 1, (0, 0, 10, 10), 2);
    open(&mut scene, 2, (0, 0, 10, 10), 2);
    open(&mut scene, 3, (0, 0, 10, 10), 2);
    assert_eq!(order(&scene), vec![1, 2, 3]);
}

#[test]
fn the_last_raised_window_draws_on_top() {
    let mut scene = SceneTable::new();
    open(&mut scene, 1, (0, 0, 10, 10), 2);
    open(&mut scene, 2, (0, 0, 10, 10), 2);
    raise_by_pid(&mut scene, 1);
    assert_eq!(order(&scene), vec![2, 1]);
}

/// The case focus alone got wrong. Windows 1, 2 and 3 open in that order, then
/// the user clicks 1 and then 2. The window manager's stack is now 3, 1, 2 from
/// the bottom. Lifting only the focused window drew 1, 3, 2: where 1 and 3
/// overlap, 3 was drawn on top while every click there went to 1.
#[test]
fn three_windows_keep_the_order_of_their_raises() {
    let mut scene = SceneTable::new();
    open(&mut scene, 1, (0, 0, 30, 20), 2);
    open(&mut scene, 2, (34, 0, 30, 20), 2);
    open(&mut scene, 3, (10, 10, 30, 20), 2);
    raise_by_pid(&mut scene, 1);
    raise_by_pid(&mut scene, 2);
    assert_eq!(order(&scene), vec![3, 1, 2]);
    let screen = Screen::composed(&scene);
    assert_eq!(screen.top_at(15, 15), 1, "1 was raised after 3, so 1 shows where they overlap");
}

#[test]
fn a_move_keeps_a_windows_place_in_the_stack() {
    let mut scene = SceneTable::new();
    open(&mut scene, 1, (0, 0, 10, 10), 2);
    open(&mut scene, 2, (5, 5, 10, 10), 2);
    open(&mut scene, 1, (20, 20, 10, 10), 2);
    assert_eq!(order(&scene), vec![1, 2], "a resubmit is not a raise");
}

#[test]
fn a_raise_stays_inside_its_band() {
    let mut scene = SceneTable::new();
    open(&mut scene, 10, (0, 0, W, H), 1);
    open(&mut scene, 1, (0, 0, 20, 20), 2);
    open(&mut scene, 2, (0, 0, 20, 20), 2);
    open(&mut scene, 30, (0, 0, 8, 8), 3);
    raise_by_pid(&mut scene, 10);
    raise_by_pid(&mut scene, 1);
    assert_eq!(order(&scene), vec![10, 2, 1, 30]);
}

#[test]
fn the_stack_counter_survives_running_out() {
    let mut scene = SceneTable::new();
    scene.set_next_stack(u32::MAX - 2);
    open(&mut scene, 1, (0, 0, 10, 10), 2);
    open(&mut scene, 2, (0, 0, 10, 10), 2);
    open(&mut scene, 3, (0, 0, 10, 10), 2);
    raise_by_pid(&mut scene, 1);
    raise_by_pid(&mut scene, 2);
    assert_eq!(order(&scene), vec![3, 1, 2]);
    let top = scene.layers().map(|l| l.stack).max().expect("layers");
    assert!(top < 16, "stamps were renumbered low, top is {top}");
}

/// The window manager's rule, modelled: z is the order of raises, and the
/// window hit at a point is the one with the highest z that contains it.
/// After any sequence of opens and raises the window drawn on top at every
/// point is that window.
#[test]
fn the_window_drawn_on_top_is_the_one_a_click_there_lands_on() {
    for seed in 1..=64u64 {
        let mut rng = Rng::new(seed);
        let mut scene = SceneTable::new();
        // (owner, rect, z as the window manager would have it)
        let mut wm: Vec<WmEntry> = Vec::new();
        let mut z = 0u32;
        for owner in 1..=6u32 {
            let (w, h) = (6 + rng.below(30), 6 + rng.below(24));
            let r = (rng.below(W - w), rng.below(H - h), w, h);
            open(&mut scene, owner, r, 2);
            z += 1;
            wm.push((owner, r, z));
        }
        for _ in 0..60 {
            let owner = 1 + rng.below(6);
            raise_by_pid(&mut scene, owner);
            z += 1;
            wm.iter_mut().find(|w| w.0 == owner).expect("open").2 = z;
            let screen = Screen::composed(&scene);
            for y in 0..H {
                for x in 0..W {
                    let hit = wm
                        .iter()
                        .filter(|(_, r, _)| x >= r.0 && x < r.0 + r.2 && y >= r.1 && y < r.1 + r.3)
                        .max_by_key(|w| w.2)
                        .map_or(0, |w| w.0);
                    assert_eq!(screen.top_at(x, y), hit, "seed {seed} at ({x}, {y})");
                }
            }
        }
    }
}
