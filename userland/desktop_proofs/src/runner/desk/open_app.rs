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

//! A window opened on the desk, and a layer submitted for it.

use super::super::drag::DragState;
use super::consts::APP_LAYER_Z;
use super::types::{App, Desk};
use crate::geometry::{clamp_to_display, Rect};
use crate::scene_raise::raise_by_pid;
use crate::window::{Kind, Visibility, Window};
use crate::z_order::raise;

impl Desk {
    /// window_open (a new z, focus for a normal window), then the app's first
    /// scene submit.
    pub fn open(&mut self, pid: u32, r: (u32, u32, u32, u32), accessory_w: u32) {
        self.open_window(pid, r, accessory_w);
        self.first_submit(pid);
    }

    /// The window manager's half of an open: window_open, with no layer in
    /// the scene yet.
    pub fn open_window(&mut self, pid: u32, r: (u32, u32, u32, u32), accessory_w: u32) {
        let window_id = 0x5749_4E00 | pid;
        let rect = clamp_to_display(
            Rect { x: r.0, y: r.1, width: r.2, height: r.3 },
            self.width,
            self.height,
        );
        let window = Window {
            owner_pid: pid,
            window_id,
            rect,
            kind: Kind::Normal,
            visibility: Visibility::Visible,
            z: self.z.allocate(),
            in_use: true,
            full_screen: false,
            parent: 0,
            grab: false,
        };
        self.windows.insert(window).expect("room for the window");
        self.focus.set(pid, window_id);
        self.apps.push(App {
            pid,
            window_id,
            x: rect.x,
            y: rect.y,
            w: rect.width,
            h: rect.height,
            accessory_w,
            drag: DragState::new(),
            accessory_events: 0,
            moves: 0,
            opened: (rect.width, rect.height),
            saved: None,
        });
    }

    /// The app's half: its first scene submit, at the rect it was opened at.
    pub fn first_submit(&mut self, pid: u32) {
        let a = self.app(pid);
        let (x, y, w, h) = (a.x, a.y, a.w, a.h);
        self.submit(pid, x, y, w, h);
        self.frame();
    }

    /// window_raise from the app, told to the compositor when it restacked.
    pub fn window_raise(&mut self, pid: u32) {
        let wid = 0x5749_4E00 | pid;
        if raise(&mut self.windows, &mut self.z, pid, wid) == Some(true) && !self.compositor_lost {
            if let Some(r) = raise_by_pid(&mut self.scene, pid) {
                self.damage.accumulate(r);
            }
        }
        self.frame();
    }

    pub(super) fn submit(&mut self, pid: u32, x: u32, y: u32, w: u32, h: u32) {
        self.submit_band(pid, x, y, w, h, APP_LAYER_Z);
    }
}
