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
//! A window's surface placed with the compositor as a layer of its own,
//! named by its surface handle and its window manager window, so each of a
//! guest's windows stacks and goes on its own (OP_SCENE_SUBMIT_LAYER).

use nonos_app_skeleton::clients::compositor::{scene_submit_layer, KeyedLayer};

use super::window::APP_LAYER_Z;

/// Surface `handle` placed at `at`, `w` by `h`, as window `window`'s
/// layer; in the place of the layer of handle `replaces` (its old surface,
/// a new shape), or 0 for none. The same handle again moves it.
pub fn submit(
    (comp, rid): (u32, u32),
    handle: u64,
    at: (u32, u32),
    (w, h): (u32, u32),
    (window, replaces): (u32, u64),
) -> Result<(), &'static str> {
    let (x, y) = at;
    let layer = KeyedLayer {
        surface_handle: handle,
        x,
        y,
        width: w,
        height: h,
        z: APP_LAYER_Z,
        window_id: window,
        replaces,
    };
    scene_submit_layer(comp, rid, &layer)
}
