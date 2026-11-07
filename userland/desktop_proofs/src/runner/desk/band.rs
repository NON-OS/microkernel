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

//! A layer placed in its band, the stack restored, and a frame composed.

use super::types::Desk;
use crate::scene::Layer;
use crate::scene_raise::raise_window;
use crate::scene_submit::submit_layer;
use crate::z_order::bottom_up;

impl Desk {
    pub(super) fn submit_band(&mut self, pid: u32, x: u32, y: u32, w: u32, h: u32, z: u32) {
        let layer = Layer {
            owner_pid: pid,
            surface_handle: pid as u64,
            x,
            y,
            width: w,
            height: h,
            z,
            stack: 0,
            in_use: true,
            window_id: 0,
            keyed: false,
            miss_count: 0,
            cursor: None,
            content: Default::default(),
            opaque: false,
        };
        for r in
            submit_layer(&mut self.scene, layer).expect("room for the layer").into_iter().flatten()
        {
            self.damage.accumulate(r);
        }
    }

    /// The window manager's owed restack (server/tell_compositor.rs resync):
    /// every window showing lifted, bottom of the stack first. The desk's
    /// clients submit the old way, so each lift raises its owner's layers.
    pub fn restack(&mut self) {
        let mut lifted = Vec::new();
        bottom_up(&self.windows, |pid, window_id| {
            lifted.push((pid, window_id));
            true
        });
        for (pid, window_id) in lifted {
            if let Some(r) = raise_window(&mut self.scene, pid, window_id) {
                self.damage.accumulate(r);
            }
        }
        self.frame();
    }

    /// One composed frame: every damaged rectangle repainted, nothing else.
    pub(super) fn frame(&mut self) {
        while let Some(r) = self.damage.drain() {
            self.paint(r);
        }
    }
}
