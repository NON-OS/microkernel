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
//! released, the pixels freed or kept, and closed in the window manager.

use core::mem;

use nonos_app_skeleton::clients::compositor::scene_remove;
use nonos_app_skeleton::clients::wm::window_close;
use nonos_app_skeleton::discover::lookup_port;
use nonos_libc::mk_surface_release;

use super::super::scene_pixels::Pixels;
use super::super::window_life::Steps;

pub(super) struct Calls {
    pub(super) handle: u64,
    pub(super) window: Option<u32>,
    pub(super) pixels: Option<Pixels>,
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
        // The compositor holds one layer per process, and its remove takes
        // that layer, which is this window's.
        lookup_port(b"compositor").is_some_and(|port| scene_remove(port, rid, 0).is_ok())
    }

    fn release_surface(&mut self) -> bool {
        mk_surface_release(self.handle) >= 0
    }

    fn free_pixels(&mut self) {
        self.pixels = None;
    }

    fn keep_pixels(&mut self) {
        if let Some(p) = self.pixels.take() {
            mem::forget(p);
        }
    }

    fn wm_close(&mut self) -> bool {
        let Some(id) = self.window else { return true };
        let rid = self.rid();
        lookup_port(b"wm").is_some_and(|port| window_close(port, rid, id).is_ok())
    }
}
