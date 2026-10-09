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

//! The window manager as the router asks it: the focused window's owner, the
//! topmost window under a point, and focus routed to a pressed window (which
//! the window manager also raises). The target type is the router's own.

#[path = "../../../capsule_input_router/src/clients/wm/types.rs"]
mod types;

pub use types::Target;

use super::world::with;

/// The focused window's owner, 0 when no window has focus: the window
/// manager answered either way.
pub fn query_focus(_port_slot: &mut u32, _request_id: u32) -> Option<u32> {
    Some(with(|world| world.focus))
}

pub fn query_topmost(_port_slot: &mut u32, _request_id: u32, x: u32, y: u32) -> Option<Target> {
    with(|world| {
        let win = world.windows.iter().find(|win| win.contains(x, y))?;
        Some(Target {
            owner_pid: win.pid,
            window_id: win.id,
            local_x: x - win.x,
            local_y: y - win.y,
            win_x: win.x,
            win_y: win.y,
            win_w: win.w,
            win_h: win.h,
        })
    })
}

pub fn route_focus(_port: u32, _request_id: u32, target: Target) -> bool {
    with(|world| {
        let Some(i) = world.windows.iter().position(|win| win.id == target.window_id) else {
            return false;
        };
        let win = world.windows.remove(i);
        world.windows.insert(0, win);
        world.focus = target.owner_pid;
        true
    })
}
