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

//! A frame for the attach proofs, composed through the real attach cache,
//! and a window opened in the scene.

use super::attach_kernel::*;
use crate::attach::{AttachCache, MAX_ATTACH};
use crate::damage::DamageAccumulator;
use crate::scene::{Layer, SceneTable};
use crate::scene_submit::submit_layer;
use crate::sw_blitter::Surface;
use crate::tests::scene::frame_model::{layer, Screen};
use crate::visible::visible_layers;

/// One frame as `frame_pacer::tick` runs it: every damaged rectangle drained
/// and composed from the layers `visible_layers` hands back, including the
/// rectangles it damages itself while composing.
pub(crate) fn frame(
    screen: &mut Screen,
    scene: &mut SceneTable,
    cache: &mut AttachCache,
    kernel: &mut FakeKernel,
    acc: &mut DamageAccumulator,
) {
    while let Some(r) = acc.drain() {
        let mut out = [(Layer::default(), Surface::default()); MAX_ATTACH];
        let n = visible_layers(scene, cache, kernel, acc, &mut out);
        let layers: Vec<Layer> = out[..n].iter().map(|(l, _)| *l).collect();
        screen.paint_layers(&layers, r);
    }
}

pub(crate) fn open(
    scene: &mut SceneTable,
    acc: &mut DamageAccumulator,
    owner: u32,
    r: (u32, u32, u32, u32),
) {
    let l = layer(owner, r.0, r.1, r.2, r.3, 2);
    for rect in submit_layer(scene, l).expect("room for the layer").into_iter().flatten() {
        acc.accumulate(rect);
    }
}
