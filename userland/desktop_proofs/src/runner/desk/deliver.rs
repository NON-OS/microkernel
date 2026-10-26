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

//! An event routed to its window and handled by that window's frame.

use super::super::drag::{self, PointerAction};
use super::super::press_part::Route;
use super::event::event;
use super::types::Desk;
use crate::focus::topmost_hit_at;
use crate::input::{InputEvent, InputKind};
use crate::scene_raise::raise_by_pid;
use crate::z_order::raise;
use nonos_toolkit::decorations::accessory_rect;

impl Desk {
    pub(super) fn route(&mut self, kind: InputKind, x: u32, y: u32) {
        if let Some(p) = self.press {
            let (lx, ly) = p.local(x, y);
            self.deliver(p.pid, event(kind, lx, ly));
        } else if let Some(hit) = topmost_hit_at(&self.windows, x, y, self.shell) {
            let ev = event(kind, hit.local_x as i32, hit.local_y as i32);
            self.deliver(hit.owner_pid, ev);
        }
        self.frame();
    }

    /// The window's drain: click to focus on a press, the press routing, the
    /// drag, then a move applied as apply_move applies it.
    pub(super) fn deliver(&mut self, pid: u32, ev: InputEvent) {
        let Some(i) = self.apps.iter().position(|a| a.pid == pid) else { return };
        if ev.kind == InputKind::ButtonDown {
            // click_focus: the app's own raise and focus, after the router's.
            let wid = self.apps[i].window_id;
            if raise(&mut self.windows, &mut self.z, pid, wid) == Some(true)
                && !self.compositor_lost
            {
                if let Some(r) = raise_by_pid(&mut self.scene, pid) {
                    self.damage.accumulate(r);
                }
            }
            self.focus.set(pid, wid);
        }
        let app = &mut self.apps[i];
        let acc = accessory_rect(app.w, app.h, false, app.accessory_w);
        let ev = match app.drag.press.route(ev, acc) {
            Route::Accessory(_) => {
                app.accessory_events += 1;
                return;
            }
            Route::Window(ev) => ev,
        };
        let action = drag::handle(&mut app.drag, (app.w, app.h), (app.x, app.y), false, &ev);
        match action {
            PointerAction::MoveTo(nx, ny) => self.apply_move(i, nx, ny),
            PointerAction::ResizeTo(nw, nh) => self.apply_resize(i, nw, nh),
            PointerAction::Place(p) => self.apply_place(i, p),
            _ => {}
        }
    }
}
