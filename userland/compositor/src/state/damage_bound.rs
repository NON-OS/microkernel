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

//! A client's damage bounded by its own surfaces.
//!
//! DAMAGE_COMMIT used to take any rectangle on the canvas, so one client
//! could make the compositor compose again pixels that only other clients'
//! surfaces show, as often as it liked: work it could make for every frame,
//! and timing it could watch for. A client's damage now counts only where
//! its own layers are. Nothing it could change is lost by that: its pixels
//! are on its layers, and moving or removing a layer damages the place it
//! left in the step that moves it (scene_submit.rs, scene_remove.rs).

use super::damage::Rect;
use super::scene::Layer;

/// The parts of `r` on `owner`'s layers, one per layer (four at most).
pub fn by_owner<'a>(
    r: Rect,
    layers: impl Iterator<Item = &'a Layer> + 'a,
    owner: u32,
) -> impl Iterator<Item = Rect> + 'a {
    layers.filter(move |l| l.owner_pid == owner).filter_map(move |l| r.intersect(of(l)))
}

fn of(l: &Layer) -> Rect {
    Rect { x: l.x, y: l.y, width: l.width, height: l.height }
}
