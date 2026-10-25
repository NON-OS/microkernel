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

//! A pointer event while a press is held: the gesture stays with the window
//! it began on, whatever is under the pointer by then.

use nonos_libc::{InputEvent, INPUT_KIND_BUTTON_DOWN, INPUT_KIND_BUTTON_UP};

use crate::state::Context;

use super::route_to_press::route_to_press;

/// What was delivered when a press held the event, or None to route it by
/// what is under the pointer.
pub(super) fn press_path(ctx: &mut Context, event: &InputEvent, x: u32, y: u32) -> Option<u32> {
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
    ctx.press.as_ref()?;
    let delivered = route_to_press(ctx, event, x, y);
    if event.kind == INPUT_KIND_BUTTON_UP {
        if let Some(press) = ctx.press.as_mut() {
            press.lift(event.code);
            if press.idle() {
                ctx.press = None;
            }
        }
    }
    Some(delivered)
}
