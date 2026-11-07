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

use super::damage::Rect;
use super::scene::{Layer, Refused, SceneTable};

// Puts `layer` in the scene and returns the rectangles to repaint: where the
// owner's layer is now, and where it was when the submit moved or resized it.
//
// A moved window uncovers what lay under its old place. Only the new place
// used to be damaged here, so a client that did not also commit its old
// rectangle left a copy of the window behind, and even one that did sent it
// as a second message: a frame composed between the two showed the window in
// both places. Damaging both here, in the one step that moves the layer,
// leaves no such frame.
//
// It fails only when the scene is full, which the handler answers E_INVAL.
pub fn submit_layer(scene: &mut SceneTable, layer: Layer) -> Result<[Option<Rect>; 2], Refused> {
    let before = scene
        .layers()
        .find(|l| !l.keyed && l.owner_pid == layer.owner_pid && l.z == layer.z)
        .map(rect_of);
    scene.submit(layer)?;
    let now = rect_of(&layer);
    let was = before.filter(|r| !same(*r, now));
    Ok([Some(now), was])
}

pub fn rect_of(layer: &Layer) -> Rect {
    Rect { x: layer.x, y: layer.y, width: layer.width, height: layer.height }
}

fn same(a: Rect, b: Rect) -> bool {
    a.x == b.x && a.y == b.y && a.width == b.width && a.height == b.height
}

/// What a keyed submit changed.
#[derive(Clone, Copy)]
pub struct Keyed {
    /// Where the layer is now, and where it was when it moved or resized.
    pub repaint: [Option<Rect>; 2],
    /// The surface the layer showed until now, when it took another; the
    /// caller lets go of its mapping unless another layer still shows it.
    pub released: Option<u64>,
}

// The keyed submit (scene/submit_keyed.rs) with the same repaint rule as
// submit_layer: both the new place and the old one are damaged in the step
// that moves the layer, so no frame shows the window in both places.
pub fn submit_keyed_layer(
    scene: &mut SceneTable,
    layer: Layer,
    replaces: Option<u64>,
) -> Result<Keyed, Refused> {
    let was = scene.submit_keyed(layer, replaces)?;
    let now = rect_of(&layer);
    let moved = was.map(|w| rect_of(&w)).filter(|r| !same(*r, now));
    let released = was.map(|w| w.surface_handle).filter(|&h| h != layer.surface_handle);
    let released = released.filter(|&h| !scene.layers().any(|l| l.surface_handle == h));
    Ok(Keyed { repaint: [Some(now), moved], released })
}
