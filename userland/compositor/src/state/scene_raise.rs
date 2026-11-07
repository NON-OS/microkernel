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
use super::scene::{Layer, SceneTable};
use super::scene_submit::rect_of;

// Raises `owner_pid`'s layer to the top of its band and returns the rectangle
// to repaint, or None when nothing on the screen changed.
//
// Only where a raised layer meets a layer it rose over can a pixel change:
// everywhere else the same layer is on top as before. The whole screen used
// to be recomposited on every focus change, and then the raised layer's whole
// rectangle; a click on a window half under another repainted all of it, a
// full window between the press that raises it and the first step of the
// drag that press starts.
pub fn raise_by_pid(scene: &mut SceneTable, owner_pid: u32) -> Option<Rect> {
    let mut union: Option<Rect> = None;
    scene.raise(owner_pid, |m, l| union = uncovered(union, m, l));
    union
}

// One window's layers on top of their bands (scene/raise.rs raise_window),
// and the area that raise can change on the screen.
pub fn raise_window(scene: &mut SceneTable, owner_pid: u32, window_id: u32) -> Option<Rect> {
    let mut union: Option<Rect> = None;
    scene.raise_window(owner_pid, window_id, |m, l| union = uncovered(union, m, l));
    union
}

// `union` grown by where raised layer `m` meets `l`, which stood over it.
fn uncovered(union: Option<Rect>, m: &Layer, l: &Layer) -> Option<Rect> {
    match rect_of(m).intersect(rect_of(l)) {
        Some(r) => Some(union.map_or(r, |u| u.union(r))),
        None => union,
    }
}
