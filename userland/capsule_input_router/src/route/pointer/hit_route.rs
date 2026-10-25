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

//! A press, release, touch or wheel routed by what is under the pointer: the
//! topmost window, or the shell for the desk, the dock or no window at all.

use nonos_libc::{InputEvent, INPUT_KIND_BUTTON_DOWN};

use crate::state::{Context, Press};

use super::route_to_shell::route_to_shell;
use super::route_to_window::route_to_window;
use super::shell_pid::shell_pid;
use super::topmost_target::topmost_target;

pub(super) fn hit_route(ctx: &mut Context, event: &InputEvent, x: u32, y: u32) -> u32 {
    let target = topmost_target(ctx, x, y);
    let shell = shell_pid(ctx);
    match target {
        Some(target) if target.owner_pid != shell => {
            if event.kind == INPUT_KIND_BUTTON_DOWN {
                let mut press = Press::arm(target.owner_pid, x, y, target.local_x, target.local_y);
                press.hold(event.code);
                ctx.press = Some(press);
            }
            route_to_window(ctx, event, target)
        }
        // A press on the desk, the dock or no window at all is the shell's,
        // and so is its release wherever the pointer is by then. The shell
        // grabs the pointer for an icon drag only once it has read the press;
        // a tap's release is routed before that, and without this grab it was
        // dropped, leaving the icon on the pointer.
        _ => {
            if event.kind == INPUT_KIND_BUTTON_DOWN && shell != 0 {
                let mut press = Press::arm(shell, x, y, x, y);
                press.hold(event.code);
                ctx.press = Some(press);
            }
            route_to_shell(ctx, event, x, y)
        }
    }
}
