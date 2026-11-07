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

//! The calls that take a guest's window down, one step each as
//! window_life.rs orders them: off the compositor's scene, the surface
//! buffers released, their pixels freed or kept, and closed in the window
//! manager.


use nonos_app_skeleton::clients::compositor::scene_remove_layer;
use nonos_app_skeleton::clients::wm::window_close;
use nonos_app_skeleton::discover::lookup_port;

use super::super::scene_buffers::Buffers;
use super::super::window_life::Steps;

pub(super) struct Calls {
    pub(super) handle: u64,
    pub(super) window: Option<u32>,
    pub(super) bufs: Option<Buffers>,
    pub(super) serial: u32,
}

impl Calls {
    fn rid(&mut self) -> u32 {
        self.serial = self.serial.wrapping_add(1);
        self.serial
    }
}

impl Steps for Calls {
    fn scene_remove(&mut self) -> bool {
        let rid = self.rid();
        // This window's layer alone: the guest's other windows stay.
        let (handle, z) = (self.handle, super::super::present::window::APP_LAYER_Z);
        lookup_port(b"compositor")
            .is_some_and(|port| scene_remove_layer(port, rid, handle, z).is_ok())
    }

    /// Both buffers' surfaces; one the kernel would not let go of keeps
    /// its pixels for good, the other's are freed (scene_buffers.rs).
    fn release_surface(&mut self) -> bool {
        self.bufs.as_mut().is_none_or(Buffers::release)
    }

    fn free_pixels(&mut self) {
        self.bufs = None;
    }

    fn keep_pixels(&mut self) {
        // What release kept, it keeps; the rest goes.
        self.bufs = None;
    }

    fn wm_close(&mut self) -> bool {
        let Some(id) = self.window else { return true };
        let rid = self.rid();
        lookup_port(b"wm").is_some_and(|port| window_close(port, rid, id).is_ok())
    }
}
