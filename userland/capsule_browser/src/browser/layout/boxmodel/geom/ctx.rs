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

use super::containing::Containing;

/* Ambient layout state a box hands to its children: the percentage bases,
 * the active clip, the stacking z and the viewport. */
#[derive(Clone, Copy)]
pub(crate) struct Ctx {
    pub cb: Containing,
    pub clip: Option<[i32; 4]>,
    pub z: i32,
    /* True inside a position:fixed subtree; its fragments pin on scroll. */
    pub fixed: bool,
    /* Inside a sticky subtree: the sticky box's flow y and its top offset,
     * so paint can clamp the whole subtree with one shift. */
    pub sticky: Option<(i32, i32)>,
    /* Accumulated opacity of the ancestors, 255 fully opaque. */
    pub alpha: u8,
    /* The viewport (width, height): the containing block of a fixed box. */
    pub vp: (i32, i32),
    /* The border-box size an out-of-flow box was given by its insets. Taken
     * by the box itself, never seen by its children. */
    pub pin: Option<Pin>,
}

/* A border-box width, and a height when the insets fixed one. */
#[derive(Clone, Copy)]
pub(crate) struct Pin {
    pub w: i32,
    pub h: Option<i32>,
}

impl Ctx {
    /* Intersect the active clip with `rect` ([x0, y0, x1, y1]). */
    pub(crate) fn clipped(mut self, rect: [i32; 4]) -> Self {
        self.clip = Some(match self.clip {
            Some(c) => [c[0].max(rect[0]), c[1].max(rect[1]), c[2].min(rect[2]), c[3].min(rect[3])],
            None => rect,
        });
        self
    }
}
