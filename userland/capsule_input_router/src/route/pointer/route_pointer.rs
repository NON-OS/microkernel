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

use nonos_libc::{
    InputEvent, INPUT_KIND_BUTTON_DOWN, INPUT_KIND_BUTTON_UP, INPUT_KIND_POINTER_ABS,
    INPUT_KIND_POINTER_REL, INPUT_KIND_TOUCH, INPUT_KIND_WHEEL,
};

use crate::state::{Context, Press};

use super::hover_motion::hover_motion;
use super::mirror_shell_pointer::mirror_shell_pointer;
use super::refresh_display::refresh_display;
use super::route_to_press::route_to_press;
use super::route_to_shell::route_to_shell;
use super::route_to_window::route_to_window;
use super::shell_pid::shell_pid;
use super::topmost_target::topmost_target;

pub fn route_pointer(ctx: &mut Context, event: &InputEvent) -> u32 {
    refresh_display(ctx);
    let (x, y) = ctx.cursor.apply(event);
    ctx.cursor_x = x;
    ctx.cursor_y = y;
    ctx.cursor_dirty = true;
    let mut delivered = mirror_shell_pointer(ctx, event, x, y);
    // A second button pressed while the grab is held joins it, so the pressed
    // window keeps the whole gesture. The same button pressed again means its
    // release was lost (device desync, capsule restart): drop the stale grab
    // and let the new press hit-test normally, instead of routing every click
    // to a dead window from then on.
    if event.kind == INPUT_KIND_BUTTON_DOWN {
        if let Some(press) = ctx.press.as_mut() {
            if !press.hold(event.code) {
                ctx.press = None;
            }
        }
    }
    // Any press can restack windows: one on a window raises it, one on the
    // dock brings another up. The cached hover window may no longer be the
    // one on top under the pointer, so forget it and ask again.
    if event.kind == INPUT_KIND_BUTTON_DOWN {
        ctx.hover = None;
    }
    if ctx.press.is_some() {
        delivered += route_to_press(ctx, event, x, y);
        if event.kind == INPUT_KIND_BUTTON_UP {
            if let Some(press) = ctx.press.as_mut() {
                press.lift(event.code);
                if press.idle() {
                    ctx.press = None;
                }
            }
        }
        ctx.record(delivered);
        return delivered;
    }
    if is_motion(event.kind) {
        delivered += hover_motion(ctx, event, x, y);
    }
    if needs_hit_test(event.kind) {
        let target = topmost_target(ctx, x, y);
        let shell = shell_pid(ctx);
        delivered += match target {
            Some(target) if target.owner_pid != shell => {
                if event.kind == INPUT_KIND_BUTTON_DOWN {
                    let mut press =
                        Press::arm(target.owner_pid, x, y, target.local_x, target.local_y);
                    press.hold(event.code);
                    ctx.press = Some(press);
                }
                route_to_window(ctx, event, target)
            }
            // A press on the desk, the dock or no window at all is the
            // shell's, and so is its release wherever the pointer is by then.
            // The shell grabs the pointer for an icon drag only once it has
            // read the press; a tap's release is routed before that, and
            // without this grab it was dropped, leaving the icon on the pointer.
            _ => {
                if event.kind == INPUT_KIND_BUTTON_DOWN && shell != 0 {
                    let mut press = Press::arm(shell, x, y, x, y);
                    press.hold(event.code);
                    ctx.press = Some(press);
                }
                route_to_shell(ctx, event, x, y)
            }
        };
    }
    ctx.record(delivered);
    delivered
}

fn is_motion(kind: u16) -> bool {
    kind == INPUT_KIND_POINTER_REL || kind == INPUT_KIND_POINTER_ABS
}

fn needs_hit_test(kind: u16) -> bool {
    matches!(
        kind,
        INPUT_KIND_BUTTON_DOWN | INPUT_KIND_BUTTON_UP | INPUT_KIND_TOUCH | INPUT_KIND_WHEEL
    )
}
