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

//! A window whose owner exits without a scene remove leaves the screen at
//! once, and the compositor never draws from a mapping the kernel has taken
//! back. The kernel here keeps the set of surfaces it still knows, as the
//! surface registry does: an owner's exit drops every record of its surfaces.

use super::attach_kernel::*;
use super::attach_rig::*;
use crate::attach::{AttachCache, MAX_ATTACH};
use crate::damage::{DamageAccumulator, Rect};
use crate::scene::{Layer, SceneTable};
use crate::sw_blitter::Surface;
use crate::tests::scene::frame_model::Screen;
use crate::visible::visible_layers;

#[test]
fn a_window_whose_owner_died_leaves_the_screen_in_the_next_frame() {
    let (mut scene, mut acc, mut cache) =
        (SceneTable::new(), DamageAccumulator::new(), AttachCache::new());
    let mut kernel = FakeKernel::default();
    kernel.live.extend([1, 2]);
    open(&mut scene, &mut acc, 1, (2, 2, 40, 30));
    open(&mut scene, &mut acc, 2, (20, 14, 40, 30));
    let mut screen = Screen::composed(&scene);
    frame(&mut screen, &mut scene, &mut cache, &mut kernel, &mut acc);
    assert_eq!(screen.top_at(30, 20), 2);

    // Owner 2 crashes: the kernel forgets its surface, and the compositor
    // only learns of it from its own lookups. The cursor moves far away.
    kernel.live.remove(&2);
    acc.accumulate(Rect { x: 0, y: 44, width: 4, height: 4 });
    frame(&mut screen, &mut scene, &mut cache, &mut kernel, &mut acc);

    assert!(scene.layers().all(|l| l.owner_pid != 2), "the dead window's layer is gone");
    let full = Screen::composed(&scene);
    if let Some((x, y, shown, wanted)) = screen.first_difference(&full) {
        panic!("pixel ({x}, {y}) still shows owner {shown}, a full frame shows {wanted}");
    }
    assert_eq!(screen.top_at(30, 20), 1, "the window under it shows again");
}

#[test]
fn a_mapped_surface_is_checked_with_the_kernel_on_every_composite() {
    let (mut scene, mut acc, mut cache) =
        (SceneTable::new(), DamageAccumulator::new(), AttachCache::new());
    let mut kernel = FakeKernel::default();
    kernel.live.insert(5);
    open(&mut scene, &mut acc, 5, (0, 0, 8, 8));
    let mut out = [(Layer::default(), Surface::default()); MAX_ATTACH];
    for round in 1..=3 {
        assert_eq!(visible_layers(&mut scene, &mut cache, &mut kernel, &mut acc, &mut out), 1);
        assert_eq!(kernel.attaches, round, "composite {round} asked the kernel");
    }
    kernel.live.remove(&5);
    assert_eq!(visible_layers(&mut scene, &mut cache, &mut kernel, &mut acc, &mut out), 0);
    assert_eq!(kernel.released, vec![5], "the dead surface is released, freeing its frames");
}
