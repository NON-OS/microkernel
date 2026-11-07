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

//! A window moved by a submit has its old and new places damaged in that
//! one step, so a client that also commits damage over the place it left
//! adds nothing: that damage counts only on its own layers, which are now
//! at the new place the submit already damaged. app_skeleton's move sends
//! the submit alone.

use crate::damage::{DamageAccumulator, Rect};
use crate::damage_bound::by_owner;
use crate::scene::SceneTable;
use crate::scene_submit::submit_layer;
use crate::tests::scene::frame_model::layer;

fn inside(p: Rect, r: Rect) -> bool {
    p.x >= r.x && p.y >= r.y && p.right() <= r.right() && p.bottom() <= r.bottom()
}

#[test]
fn a_moves_submit_damages_all_a_commit_of_the_old_place_could() {
    let moves = [((4, 4), (20, 9)), ((4, 4), (6, 5)), ((30, 20), (2, 1)), ((10, 10), (10, 30))];
    for ((ox, oy), (nx, ny)) in moves {
        let mut scene = SceneTable::new();
        let _ = submit_layer(&mut scene, layer(7, 40, 30, 12, 12, 2));
        let _ = submit_layer(&mut scene, layer(1, ox, oy, 16, 12, 2));
        let mut acc = DamageAccumulator::new();
        for r in submit_layer(&mut scene, layer(1, nx, ny, 16, 12, 2)).expect("room") {
            r.into_iter().for_each(|r| acc.accumulate(r));
        }
        let old = Rect { x: ox, y: oy, width: 16, height: 12 };
        let new = Rect { x: nx, y: ny, width: 16, height: 12 };
        let mut damaged = Vec::new();
        while let Some(r) = acc.drain() {
            damaged.push(r);
        }
        assert!(damaged.iter().any(|d| inside(old, *d)), "the old place is damaged");
        assert!(damaged.iter().any(|d| inside(new, *d)), "the new place is damaged");
        for (_, part) in by_owner(old, scene.layers(), 1) {
            let covered = damaged.iter().any(|d| inside(part, *d));
            assert!(
                covered,
                "({}, {}) of a commit over the old place was not damaged",
                part.x, part.y
            );
            assert!(inside(part, new), "a commit over the old place counts only on the new");
        }
    }
}
