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

//! A press, a motion and a release, from the router's side.

use super::event::event;
use super::types::Desk;
use crate::focus::{press_focus, topmost_hit_at};
use crate::input::InputKind;
use crate::press::Press;
use crate::scene_raise::raise_by_pid;

impl Desk {
    /// A button press at screen (x, y), as route_pointer handles it.
    pub fn press(&mut self, x: u32, y: u32) {
        self.press = None;
        // No window, or the shell's own: route_to_shell, no focus change.
        let hit =
            topmost_hit_at(&self.windows, x, y, self.shell).filter(|h| h.owner_pid != self.shell);
        let Some(hit) = hit else {
            self.shell_presses += 1;
            return;
        };
        // route_to_window: route_focus first (the window manager focuses and
        // raises, and pushes focus_set when that restacked anything) ...
        let pressed = press_focus(
            &mut self.windows,
            &mut self.z,
            &mut self.focus,
            hit.owner_pid,
            hit.window_id,
        )
        .expect("the hit window takes focus");
        if pressed.restacked() && !self.compositor_lost {
            if let Some(r) = raise_by_pid(&mut self.scene, hit.owner_pid) {
                self.damage.accumulate(r);
            }
        }
        // ... then the press grab, then the press in window coordinates.
        self.press = Some(Press::arm(hit.owner_pid, x, y, hit.local_x, hit.local_y));
        let ev = event(InputKind::ButtonDown, hit.local_x as i32, hit.local_y as i32);
        self.deliver(hit.owner_pid, ev);
        self.frame();
    }

    /// Pointer motion to screen (x, y): through the press grab while a button
    /// is down, else to the window under the pointer in its own coordinates.
    pub fn motion(&mut self, x: u32, y: u32) {
        self.route(InputKind::PointerAbs, x, y);
        if self.shell != 0 {
            self.shell_motion(y);
        }
    }

    pub fn release(&mut self, x: u32, y: u32) {
        self.route(InputKind::ButtonUp, x, y);
        self.press = None;
    }
}
