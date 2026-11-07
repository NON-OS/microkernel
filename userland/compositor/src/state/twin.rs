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

//! A layer moved to another band as its owner's twin: the same surface at
//! the same place placed in the new band, then the old band's layer
//! dropped. The desktop shell moves its chrome into the trusted band and out
//! of it this way, so it is never in neither (its server/trusted_band.rs).
//!
//! Each step composed the whole screen, three frames for the lock screen's
//! curtain going up and three for it coming down. When the twin left in
//! place is opaque, a step changes only where layers between the two bands
//! are drawn: the same pixels are on top before and after everywhere else.
//! A twin placed under an opaque twin, or dropped from under one, changes
//! nothing at all (hidden.rs).

use super::damage::{DamageAccumulator, Rect};
use super::scene::{Layer, SceneTable};
use super::scene_submit::rect_of;

/// Composite rank, as scene/snapshot.rs draws.
fn rank(l: &Layer) -> u64 {
    ((l.z as u64) << 32) | l.stack as u64
}

fn same(a: Rect, b: Rect) -> bool {
    (a.x, a.y, a.width, a.height) == (b.x, b.y, b.width, b.height)
}

/// `l`'s twin in another band: its owner's layer showing the same surface at
/// the same place with the same content, opaque and drawn.
pub fn opaque_twin(scene: &SceneTable, l: &Layer) -> Option<Layer> {
    let twin = |t: &&Layer| {
        t.owner_pid == l.owner_pid
            && t.surface_handle == l.surface_handle
            && t.z != l.z
            && same(rect_of(t), rect_of(l))
            && t.content == l.content
            && t.opaque
            && t.miss_count == 0
    };
    scene.layers().find(twin).copied()
}

/// Where the screen changes when `top`, a twin of the opaque layer `under`
/// drawn below it, is shown or no longer shown: the parts of their place
/// where a layer ranked between the two is drawn, which `top` covers. `top`
/// is opaque too, so nothing under it shows through either way.
pub fn between<'a>(
    scene: &'a SceneTable,
    under: &'a Layer,
    top: &'a Layer,
) -> impl Iterator<Item = Rect> + 'a {
    let (lo, hi, place) = (rank(under), rank(top), rect_of(top));
    scene
        .layers()
        .filter(move |l| rank(l) > lo && rank(l) < hi)
        .filter_map(move |l| place.intersect(rect_of(l)))
}

/// `owner`'s layer just placed fresh in band `z`, showing `handle`: when it
/// is the twin of an opaque layer it shows the same pixels, so it is opaque
/// too, and only what it changes is damaged; true then. False when it is no
/// such twin, and the submit's own repaint stands.
pub fn placed(
    scene: &mut SceneTable,
    damage: &mut DamageAccumulator,
    (owner, z, handle): (u32, u32, u64),
) -> bool {
    let fresh =
        |l: &&Layer| !l.keyed && l.owner_pid == owner && l.z == z && l.surface_handle == handle;
    let Some(new) = scene.layers().find(fresh).copied() else { return false };
    let Some(twin) = opaque_twin(scene, &new) else { return false };
    scene.set_opaque(owner, handle, true);
    let new = Layer { opaque: true, ..new };
    if rank(&new) > rank(&twin) {
        between(scene, &twin, &new).for_each(|r| damage.accumulate(r));
    }
    true
}

/// `gone`, just dropped from the scene: when an opaque twin of it is left,
/// over it or under it with `gone` opaque too, only what changed is damaged;
/// true then. False otherwise, and its whole place is repainted.
pub fn dropped(scene: &SceneTable, damage: &mut DamageAccumulator, gone: &Layer) -> bool {
    let Some(twin) = opaque_twin(scene, gone) else { return false };
    if rank(gone) < rank(&twin) {
        return true;
    }
    if !gone.opaque || gone.miss_count != 0 {
        return false;
    }
    between(scene, &twin, gone).for_each(|r| damage.accumulate(r));
    true
}
