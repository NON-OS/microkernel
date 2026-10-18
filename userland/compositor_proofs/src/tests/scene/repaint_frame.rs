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

//! Frames for the repaint proofs that leave dead or blank surfaces out, as
//! the compositor leaves out a layer whose surface does not attach.

use crate::damage::{DamageAccumulator, Rect};
use crate::scene::{Layer, SceneTable};
use crate::scene_submit::rect_of;
use crate::tests::scene::frame_model::Screen;

/// What composite::paint does for one damaged rectangle, with the surfaces in
/// `dead` failing to attach: compose without them, then reap, repainting the
/// rectangle of every layer the reaper drops.
pub(crate) fn composite(
    screen: &mut Screen,
    scene: &mut SceneTable,
    acc: &mut DamageAccumulator,
    clip: Rect,
    dead: &[u64],
) {
    screen.paint_except(scene, clip, dead);
    let attached: Vec<u64> =
        scene.layers().map(|l| l.surface_handle).filter(|h| !dead.contains(h)).collect();
    let mut dropped = [Layer::default(); 32];
    let n = scene.reap_unattachable(&attached, REAP_THRESHOLD, &mut dropped);
    for l in &dropped[..n] {
        acc.accumulate(rect_of(l));
    }
}

pub(crate) const REAP_THRESHOLD: u16 = 60;

/// Compose every damaged rectangle with the surfaces in `blank` drawn as
/// what they hold: nothing yet, so whatever is under them shows.
pub(crate) fn frame_with_blank(
    screen: &mut Screen,
    scene: &SceneTable,
    acc: &mut DamageAccumulator,
    blank: &[u64],
) {
    while let Some(r) = acc.drain() {
        screen.paint_except(scene, r, blank);
    }
}
