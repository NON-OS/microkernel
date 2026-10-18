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

//! The desk driven: a layer submitted with its damage, a frame composed
//! through the compositor's own `compose`, and the pointer moved.

use super::foreign_desk::*;
use super::foreign_kernel::*;
use crate::damage::Rect;
use crate::frame_pacer::compose::{compose, Scene};
use crate::scene::Layer;
use crate::scene_submit::submit_layer;

impl Desk {
    pub(crate) fn submit(&mut self, owner: u32, handle: u64, r: (u32, u32, u32, u32), z: u32) {
        let l = Layer {
            owner_pid: owner,
            surface_handle: handle,
            x: r.0,
            y: r.1,
            width: r.2,
            height: r.3,
            z,
            stack: 0,
            in_use: true,
            miss_count: 0,
        };
        for rect in submit_layer(&mut self.scene, l).expect("room").into_iter().flatten() {
            self.damage.accumulate(rect);
        }
    }

    /// One frame: every damaged rectangle drained and composed.
    pub(crate) fn frame(&mut self) {
        while let Some(r) = self.damage.drain() {
            let dst = self.dst();
            let s = Scene {
                scene: &mut self.scene,
                attach: &mut self.attach,
                kernel: &mut self.kernel,
                damage: &mut self.damage,
                ceiling: u32::MAX,
            };
            compose(dst, r, s, Some(self.cursor));
        }
    }

    /// The pointer moves, damaging where it was and where it is, as the
    /// cursor-update handler does.
    pub(crate) fn move_cursor(&mut self, x: u32, y: u32) {
        for (cx, cy) in [self.cursor, (x, y)] {
            let w = CURSOR_SIDE.min(W - cx);
            let h = CURSOR_SIDE.min(H - cy);
            self.damage.accumulate(Rect { x: cx, y: cy, width: w, height: h });
        }
        self.cursor = (x, y);
    }
}
