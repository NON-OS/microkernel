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

//! An exited app's window memory is let go at once, and a closed window is
//! forgotten and released.

use super::attach_kernel::*;
use super::attach_rig::*;
use crate::attach::AttachCache;
use crate::damage::{DamageAccumulator, Rect};
use crate::scene::SceneTable;
use crate::tests::scene::frame_model::Screen;

/// An app that exits holding its window (no scene remove) leaves the
/// compositor's view of it as the last hold on its frames. The compositor
/// must let go in the frame that finds the owner gone, exactly once, or the
/// window's memory (8 MB for a 1080p window) is never freed.
#[test]
fn an_exited_apps_window_memory_is_let_go_at_once() {
    let (mut scene, mut acc, mut cache) =
        (SceneTable::new(), DamageAccumulator::new(), AttachCache::new());
    let mut kernel = FakeKernel::default();
    kernel.live.extend([1, 2, 3]);
    for (owner, r) in [(1, (0, 0, 20, 20)), (2, (10, 10, 20, 20)), (3, (30, 5, 20, 20))] {
        open(&mut scene, &mut acc, owner, r);
    }
    let mut screen = Screen::composed(&scene);
    frame(&mut screen, &mut scene, &mut cache, &mut kernel, &mut acc);
    assert_eq!(kernel.mapped.len(), 3);
    kernel.live.remove(&2);
    for _ in 0..5 {
        acc.accumulate(Rect { x: 0, y: 44, width: 4, height: 4 });
        frame(&mut screen, &mut scene, &mut cache, &mut kernel, &mut acc);
    }
    assert!(!kernel.mapped.contains(&2), "the dead window's frames are released");
    assert_eq!(kernel.released, vec![2], "released once, nothing else");
    assert!(scene.layers().all(|l| l.owner_pid != 2));
}

/// A normal close: the client's scene remove forgets its surface, and a
/// later lookup of anything maps nothing stale.
#[test]
fn a_closed_window_is_forgotten_and_released() {
    let mut cache = AttachCache::new();
    let mut kernel = FakeKernel::default();
    kernel.live.insert(9);
    let _ = cache.lookup(9, &[9], &mut kernel);
    assert!(cache.forget(9, &mut kernel));
    assert!(kernel.mapped.is_empty());
    let _ = cache.forget(9, &mut kernel);
    assert_eq!(kernel.released, vec![9], "released once, a second forget releases nothing");
}
