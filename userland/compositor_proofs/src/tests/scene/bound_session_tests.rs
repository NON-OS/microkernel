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

//! Bounding damage by the sender's layers loses no pixel. Windows here draw
//! new content (each pixel its owner and that owner's frame count), commit
//! damage larger than they are, move and resize the way app_skeleton does
//! (a submit, then damage where they were), and a hostile one damages the
//! whole canvas. The screen, kept only through bounded damage, must equal a
//! full frame after every step.

use crate::damage::{DamageAccumulator, Rect};
use crate::scene::SceneTable;
use crate::scene_remove::remove_by_pid;
use crate::scene_submit::{rect_of, submit_layer};
use crate::tests::scene::bound_model::{commit, compose, full};
use crate::tests::scene::frame_model::{layer, Rng, H, W};

#[test]
fn bounded_damage_never_leaves_a_stale_pixel() {
    for seed in 1..=64u64 {
        let mut rng = Rng::new(seed);
        let (mut scene, mut acc, mut frames) =
            (SceneTable::new(), DamageAccumulator::new(), [0u32; 6]);
        let mut screen = vec![0u32; (W * H) as usize];
        for step in 0..120 {
            let k = 1 + rng.below(5);
            let was = scene.layers().find(|l| l.owner_pid == k).copied();
            let (x, y, w, h) = (rng.below(W), rng.below(H), 1 + rng.below(30), 1 + rng.below(24));
            match (rng.below(4), was) {
                (0, _) | (_, None) => {
                    let _ = submit_layer(&mut scene, layer(k, x, y, w, h, 2))
                        .map(|rs| rs.into_iter().flatten().for_each(|r| acc.accumulate(r)));
                    if let Some(o) = was {
                        commit(&scene, &mut acc, rect_of(&o), k);
                    }
                }
                (1, Some(o)) => {
                    frames[k as usize] += 1;
                    let r = Rect {
                        x: o.x.saturating_sub(x % 5),
                        y: o.y.saturating_sub(y % 5),
                        width: o.width + x % 5 + w,
                        height: o.height + y % 5 + h,
                    };
                    commit(&scene, &mut acc, r, k);
                }
                (2, Some(_)) => {
                    commit(&scene, &mut acc, Rect { x: 0, y: 0, width: W, height: H }, k)
                }
                _ => acc.accumulate(remove_by_pid(&mut scene, k).unwrap_or_default()),
            }
            while let Some(r) = acc.drain() {
                compose(&scene, &frames, &mut screen, r);
            }
            assert!(screen == full(&scene, &frames), "seed {seed} step {step}: a stale pixel");
        }
    }
}
