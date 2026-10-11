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

//! A scene made empty, its serials, and where its window is.

use alloc::vec::Vec;

use super::fit::Fit;
use super::held_keys::HeldKeys;
use super::popup::stack::Popups;
use super::scene::Scene;
use super::scene_pixels::Pixels;
use super::subsurface::state::Subs;

impl Scene {
    pub fn new() -> Scene {
        Scene {
            pools: Vec::new(),
            buffers: Vec::new(),
            surfaces: Vec::new(),
            serial: 1,
            out: None,
            at: None,
            shape: None,
            window: None,
            shown: None,
            fit: Fit::default(),
            pixels: Pixels::empty(),
            pointer: None,
            keyboard: None,
            pointer_on: None,
            keyboard_on: None,
            on_output: None,
            subscribed: false,
            held: HeldKeys::new(),
            subs: Subs::default(),
            popups: Popups::default(),
        }
    }

    pub fn next_serial(&mut self) -> u32 {
        self.serial += 1;
        self.serial
    }

    /// Where the window is on screen and how large, once it is shown.
    pub fn rect(&self) -> Option<(u32, u32, u32, u32)> {
        let ((x, y), (w, h, _)) = (self.at?, self.shape?);
        Some((x, y, w, h))
    }
}

impl Default for Scene {
    fn default() -> Scene {
        Scene::new()
    }
}
