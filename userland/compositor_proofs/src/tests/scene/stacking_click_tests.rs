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

//! What is drawn on top at a point is the window the window manager's hit
//! test gives a click there.

use super::stacking_rig::*;
use crate::scene::SceneTable;
use crate::scene_raise::raise_by_pid;
use crate::tests::scene::frame_model::{Rng, Screen, H, W};

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
