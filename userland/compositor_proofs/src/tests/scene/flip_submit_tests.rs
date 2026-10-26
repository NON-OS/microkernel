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
//! The pointer's shape: drawn from its hot spot inside the damaged box, the
//! A surface flipped in: the shell submits the backing it finished in place
//! of the one shown, in the same band. The layer then reads the new backing
//! alone, from that request on, with its place in the stack and its pointer
//! shape kept; the backing it read before is free for the next frame. With
//! the shell writing only the backing not shown (desktop_proofs
//! tests/surface_flip.rs), no composite reads a frame being written.

use super::frame_util::layer;
use crate::frame_pacer::cursor_shapes::ARROW_CODE;
use crate::state::scene::SceneTable;

fn shell(handle: u64) -> crate::state::scene::Layer {
    let mut l = layer(9, 0, 0, 1920, 1080, 30);
    l.surface_handle = handle;
    l
}

#[test]
fn flip_submit_reads_the_new_backing_alone_in_the_same_place() {
    let mut scene = SceneTable::new();
    scene.submit(layer(40, 0, 0, 400, 300, 30)).unwrap();
    scene.submit(shell(0xA)).unwrap();
    scene.set_cursor(9, 0xA, Some(ARROW_CODE)).unwrap();
    let before = scene.z_sorted_snapshot();
    scene.submit(shell(0xB)).unwrap();
    let (after, n) = scene.z_sorted_snapshot();
    let mine: Vec<_> = after[..n].iter().filter(|l| l.owner_pid == 9).collect();
    assert_eq!(mine.len(), 1, "one layer, not two");
    assert_eq!(mine[0].surface_handle, 0xB, "the finished backing");
    let was = before.0[..before.1].iter().find(|l| l.owner_pid == 9).unwrap();
    assert_eq!(
        (mine[0].stack, mine[0].cursor),
        (was.stack, Some(ARROW_CODE)),
        "its place, its shape"
    );
    let order =
        |s: &[crate::state::scene::Layer]| s.iter().map(|l| l.owner_pid).collect::<Vec<_>>();
    assert_eq!(order(&after[..n]), order(&before.0[..before.1]), "the stack unchanged");
}
