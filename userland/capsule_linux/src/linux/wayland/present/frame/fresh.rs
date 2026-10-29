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

//! New pixels for a window: allocated, filled, and the shape they are
//! registered at.

use super::super::super::reshape::Shape;
use super::super::super::scene::Scene;
use super::super::super::scene_pixels::Pixels;
use super::super::source::Source;
use super::fill::{fill, window_bytes};
use super::geometry::Around;

/// New pixels for frame `a`, filled from `src`, and their shape; None,
/// said, when there is no room.
pub fn fresh(
    scene: &mut Scene,
    at: (u32, u32),
    src: &Source,
    a: Around,
) -> Option<(Pixels, Shape)> {
    let bytes = window_bytes(a, src);
    let Some(mut px) = Pixels::with_room(bytes) else {
        super::super::no_room::no_heap(bytes);
        return None;
    };
    // New pixels hold no frame yet.
    scene.frame.chrome = None;
    fill(scene, at, src, px.frame(bytes)?, a).then_some(())?;
    let shape = if a.framed() { (a.w, a.h, a.w * 4) } else { src.shape };
    Some((px, shape))
}
