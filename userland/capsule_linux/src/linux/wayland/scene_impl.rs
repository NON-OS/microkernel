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

use super::clip::Clip;
use super::held_keys::HeldKeys;
use super::popup::stack::Popups;
use super::scene::Scene;
use super::scene_window::Window;
use super::subsurface::state::Subs;

impl Scene {
    pub fn new() -> Scene {
        Scene {
            pools: Vec::new(),
            buffers: Vec::new(),
            surfaces: Vec::new(),
            serial: 1,
            win: Window::default(),
            behind: Vec::new(),
            decorations: Vec::new(),
            pointer: None,
            keyboard: None,
            pointer_on: None,
            pointer_entered: 0,
            pointer_at: (0, 0),
            cursor: 0,
            keyboard_on: None,
            on_output: Vec::new(),
            subscribed: false,
            motion_kept: super::input::latest::Latest::new(),
            refresh_ns: 0,
            presenting: Vec::new(),
            held: HeldKeys::new(),
            subs: Subs::default(),
            popups: Popups::default(),
            popup_windows: Vec::new(),
            clip: Clip::default(),
            dnd: Default::default(),
            pins: Vec::new(),
            releasing: Vec::new(),
            regions: Vec::new(),
        }
    }

    pub fn next_serial(&mut self) -> u32 {
        self.serial += 1;
        self.serial
    }

    /// Where the window is on screen and how large, once it is shown.
    pub fn rect(&self) -> Option<(u32, u32, u32, u32)> {
        let ((x, y), (w, h, _)) = (self.win.at?, self.win.shape?);
        Some((x, y, w, h))
    }
}

impl Default for Scene {
    fn default() -> Scene {
        Scene::new()
    }
}
