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

//! The window manager's hit test and the compositor's draw order agree with
//! several windows and popups per owner: every raise is told to the
//! compositor window by window (z_order::chain), each window a keyed layer of
//! its own, and at every point the window drawn on top is the window a click
//! there lands on.

use super::rig::{menu, Rng, Wm};
use crate::focus::topmost_hit_at;
use crate::scene::{Layer, SceneTable};
use crate::scene_raise::raise_window;
use crate::scene_submit::submit_keyed_layer;
use crate::z_order::{chain, raise};

const W: u32 = 160;
const H: u32 = 100;
/// Every other pixel each way: no window or popup is narrower than ten.
const STEP: usize = 2;

fn layer_of(pid: u32, id: u32, r: crate::geometry::Rect) -> Layer {
    Layer {
        owner_pid: pid,
        surface_handle: u64::from(pid) << 32 | u64::from(id),
        x: r.x,
        y: r.y,
        width: r.width,
        height: r.height,
        z: 2,
        in_use: true,
        window_id: id,
        ..Layer::default()
    }
}

fn drawn_on_top(layers: &[Layer], x: u32, y: u32) -> Option<(u32, u32)> {
    layers
        .iter()
        .rev()
        .find(|l| x >= l.x && x < l.x + l.width && y >= l.y && y < l.y + l.height)
        .map(|l| (l.owner_pid, l.window_id))
}

#[test]
fn the_window_drawn_on_top_is_the_one_a_click_lands_on() {
    for seed in 1..=16u64 {
        let mut rng = Rng::new(seed);
        let mut wm = Wm::new();
        let mut scene = SceneTable::new();
        let mut next_id = [1u32; 4];
        for step in 0..40 {
            let pid = 40 + rng.below(3);
            let open: Vec<(u32, u32)> =
                wm.table.windows().map(|w| (w.owner_pid, w.window_id)).collect();
            let roll = rng.below(10);
            // Within the compositor's share for one owner (scene MAX_LAYERS_PER_OWNER).
            let room = |pid: u32| wm.table.held_by(pid) < 12;
            if (open.is_empty() || roll < 3) && room(pid) {
                let id = next_id[(pid - 40) as usize];
                next_id[(pid - 40) as usize] += 1;
                let (w, h) = (20 + rng.below(80), 20 + rng.below(60));
                let r = (rng.below(W - w), rng.below(H - h), w, h);
                wm.window(pid, id, r);
                let at = *wm.table.find(pid, id).expect("open");
                submit_keyed_layer(&mut scene, layer_of(pid, id, at.rect), None).expect("room");
            } else if roll < 5 && !open.is_empty() {
                let (pid, parent) = open[rng.below(open.len() as u32) as usize];
                if !room(pid) {
                    continue;
                }
                let id = next_id[(pid - 40) as usize];
                next_id[(pid - 40) as usize] += 1;
                let a = (rng.below(40) as i32, rng.below(30) as i32, 10, 10);
                wm.popup(pid, id, parent, menu(a, 10 + rng.below(50), 10 + rng.below(40)), false);
                let at = *wm.table.find(pid, id).expect("open");
                submit_keyed_layer(&mut scene, layer_of(pid, id, at.rect), None).expect("room");
            } else if !open.is_empty() {
                let (pid, id) = open[rng.below(open.len() as u32) as usize];
                if raise(&mut wm.table, &mut wm.z, pid, id) == Some(true) {
                    let mut ids = [0u32; 64];
                    let n = chain(&wm.table, pid, id, &mut ids);
                    for &w in &ids[..n] {
                        raise_window(&mut scene, pid, w);
                    }
                }
            }
            let (layers, n) = scene.z_sorted_snapshot();
            for y in (0..H).step_by(STEP) {
                for x in (0..W).step_by(STEP) {
                    let hit =
                        topmost_hit_at(&wm.table, x, y, 0).map(|h| (h.owner_pid, h.window_id));
                    let top = drawn_on_top(&layers[..n], x, y);
                    assert_eq!(top, hit, "seed {seed} step {step} ({x}, {y})");
                }
            }
        }
    }
}
