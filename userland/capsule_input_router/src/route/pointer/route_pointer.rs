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

use nonos_libc::InputEvent;

use crate::state::Context;

use super::event_kinds::{is_motion, needs_hit_test};
use super::hit_route::hit_route;
use super::hover_motion::hover_motion;
use super::mirror_shell_pointer::mirror_shell_pointer;
use super::press_path::press_path;
use super::refresh_display::refresh_display;

/// Place one pointer event and deliver it: to the shell's mirror, to the
/// window holding the gesture if one does, else to what is under it.
pub fn route_pointer(ctx: &mut Context, event: &InputEvent) -> u32 {
    if !refresh_display(ctx, event) {
        // No edges yet: placed on a guess, the event would land elsewhere.
        ctx.record(0);
        return 0;
    }
    let (x, y) = ctx.cursor.apply(event);
    ctx.cursor_x = x;
    ctx.cursor_y = y;
    ctx.cursor_dirty = true;
    let mut delivered = mirror_shell_pointer(ctx, event, x, y);
    if let Some(held) = press_path(ctx, event, x, y) {
        delivered += held;
        ctx.record(delivered);
        return delivered;
    }
    if is_motion(event.kind) {
        delivered += hover_motion(ctx, event, x, y);
    }
    if needs_hit_test(event.kind) {
        delivered += hit_route(ctx, event, x, y);
    }
    ctx.record(delivered);
    delivered
}
