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

//! A swapchain present: the sender's layer in band `z` shows `handle` from
//! now on, and the part of it that changed is repainted. Nothing else about
//! the layer moves. Pure, so the host proofs hold it.

use super::damage::Rect;
use super::scene::SceneTable;

#[derive(Clone, Copy)]
pub struct Swapped {
    /// The buffer the layer showed until now, when it was another.
    pub old: Option<u64>,
    /// The changed part on the canvas, cut to the layer; None when nothing
    /// of it is on the layer.
    pub repaint: Option<Rect>,
    /// The layer on the canvas.
    pub on: Rect,
}

impl Swapped {
    /// Box `part` of the buffer, in its pixels, on the canvas and cut to the
    /// layer: what a part of the changed box repaints.
    pub fn repaint_of(&self, part: Rect) -> Option<Rect> {
        let moved = Rect {
            x: self.on.x.saturating_add(part.x),
            y: self.on.y.saturating_add(part.y),
            width: part.width,
            height: part.height,
        };
        moved.intersect(self.on)
    }
}

/// None when `owner` has no layer in band `z`.
pub fn present(
    scene: &mut SceneTable,
    owner: u32,
    z: u32,
    handle: u64,
    changed: Rect,
) -> Option<Swapped> {
    let layer = shown_by(scene, owner, z, handle)?;
    swap(scene, layer, handle, changed)
}

/// The sender's layer in band `z` that shows `on` now shows `handle`. None
/// when `owner` has no layer there showing `on`, or another layer already
/// shows `handle`: one buffer is never two windows' picture.
pub fn present_on(
    scene: &mut SceneTable,
    owner: u32,
    z: u32,
    on: u64,
    handle: u64,
    changed: Rect,
) -> Option<Swapped> {
    let mine = |l: &&super::scene::Layer| l.owner_pid == owner && l.z == z;
    let layer = *scene.layers().filter(mine).find(|l| l.surface_handle == on)?;
    let taken = scene.layers().any(|l| l.surface_handle == handle && l.surface_handle != on);
    if taken {
        return None;
    }
    swap(scene, layer, handle, changed)
}

fn swap(
    scene: &mut SceneTable,
    layer: super::scene::Layer,
    handle: u64,
    changed: Rect,
) -> Option<Swapped> {
    let old = (layer.surface_handle != handle).then_some(layer.surface_handle);
    if old.is_some() {
        let next = super::scene::Layer { surface_handle: handle, miss_count: 0, ..layer };
        if layer.keyed {
            scene.submit_keyed(next, Some(layer.surface_handle)).ok()?;
        } else {
            scene.submit(next).ok()?;
        }
    }
    let on = Rect { x: layer.x, y: layer.y, width: layer.width, height: layer.height };
    let swapped = Swapped { old, repaint: None, on };
    Some(Swapped { repaint: swapped.repaint_of(changed), ..swapped })
}

// the project's layer in band z a present of `handle` is for: the one already
// showing it, else the project's one layer submitted the old way, else its one
// keyed layer in the band. An owner holding several keyed layers there names
// the layer by a handle it shows; a guess between its windows would put one
// window's picture in another's place.
fn shown_by(scene: &SceneTable, owner: u32, z: u32, handle: u64) -> Option<super::scene::Layer> {
    let mine = || scene.layers().filter(|l| l.owner_pid == owner && l.z == z);
    if let Some(l) = mine().find(|l| l.surface_handle == handle) {
        return Some(*l);
    }
    if let Some(l) = mine().find(|l| !l.keyed) {
        return Some(*l);
    }
    let mut keyed = mine().filter(|l| l.keyed);
    let one = *keyed.next()?;
    keyed.next().is_none().then_some(one)
}
