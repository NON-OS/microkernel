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

use nonos_libc::{InputEvent, INPUT_KIND_BUTTON_UP};

use crate::state::Context;

use super::route_to_press::route_to_press;

// A grab taken while a button is down, such as a dialog that comes up while a
// window's title bar is held, takes the release. The window pressed is still
// waiting for it: without it its drag never ends, and once the grab is gone it
// follows the pointer with no button down. The release goes to it as well,
// and its press grab ends with its last button.
pub(in crate::route) fn release_under_grab(
    ctx: &mut Context,
    holder: u32,
    event: &InputEvent,
    x: u32,
    y: u32,
) -> u32 {
    if event.kind != INPUT_KIND_BUTTON_UP {
        return 0;
    }
    let Some(press) = ctx.press.as_mut() else { return 0 };
    let held = press.lift(event.code);
    let (pid, idle) = (press.pid, press.idle());
    let delivered = if held && pid != holder { route_to_press(ctx, event, x, y) } else { 0 };
    if idle {
        ctx.press = None;
    }
    delivered
}
