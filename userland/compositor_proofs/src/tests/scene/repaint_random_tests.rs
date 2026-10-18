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

//! Long random sessions: windows open in three bands, move, resize, get
//! raised and close, and the screen is only ever repainted where the steps
//! said. It must match a full frame after every single step.

use super::repaint_rig::*;
use crate::tests::scene::frame_model::{layer, Rng, Screen, H, W};
use crate::{damage::DamageAccumulator, scene::SceneTable};
use crate::{scene_raise::raise_by_pid, scene_remove::remove_by_pid, scene_submit::submit_layer};

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
