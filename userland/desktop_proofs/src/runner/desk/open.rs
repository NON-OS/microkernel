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

//! A desk made, and the desktop shell brought up on it with its dock.

use super::consts::{CLEAR, DOCK_WINDOW_ID, SHELL_CHROME_Z, SHELL_LAYER_Z};
use super::types::Desk;
use crate::damage::DamageAccumulator;
use crate::focus::FocusModel;
use crate::geometry::Rect;
use crate::scene::SceneTable;
use crate::window::{Kind, Visibility, Window, WindowTable};
use crate::z_order::ZStack;

impl Desk {
    pub fn new(width: u32, height: u32) -> Desk {
        Desk {
            width,
            height,
            windows: WindowTable::new(),
            z: ZStack::new(),
            focus: FocusModel::new(),
            scene: SceneTable::new(),
            damage: DamageAccumulator::new(),
            screen: vec![0; (width * height) as usize],
            press: None,
            apps: Vec::new(),
            shell_presses: 0,
            shell: 0,
            compositor_lost: false,
            chrome: vec![CLEAR; (width * height) as usize],
            taskbar: crate::taskbar::new_taskbar_state(),
            dock: (0, 0, 0, 0),
            dock_commits: Vec::new(),
        }
    }

    /// The desktop shell as it primes: its desk, a full-screen layer in the
    /// band under application windows (DESK_Z); its chrome in the band over
    /// them (CHROME_Z), modelled here by the one part of it that is opaque
    /// while no menu is open, the dock; and a popup window over the dock so
    /// the window manager knows the dock is there.
    pub fn open_shell(&mut self, pid: u32, dock: (u32, u32, u32, u32)) {
        self.shell = pid;
        self.submit_band(pid, 0, 0, self.width, self.height, SHELL_LAYER_Z);
        self.submit_band(pid, 0, 0, self.width, self.height, SHELL_CHROME_Z);
        self.dock = dock;
        self.paint_dock();
        let window = Window {
            owner_pid: pid,
            window_id: DOCK_WINDOW_ID,
            rect: Rect { x: dock.0, y: dock.1, width: dock.2, height: dock.3 },
            kind: Kind::Popup,
            visibility: Visibility::Visible,
            z: self.z.allocate(),
            in_use: true,
            full_screen: false,
        };
        self.windows.insert(window).expect("room for the dock");
        self.frame();
    }
}
