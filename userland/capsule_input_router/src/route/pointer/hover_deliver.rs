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

//! What a hovered window is sent: the motion in its own frame, and (-1, -1)
//! when the pointer leaves it.

use nonos_libc::{InputEvent, INPUT_KIND_POINTER_ABS};

use crate::state::{Context, Hover};

use super::super::deliver::deliver_to;

pub(super) fn deliver_local(
    ctx: &mut Context,
    hover: Hover,
    event: &InputEvent,
    x: u32,
    y: u32,
) -> u32 {
    let mut routed = *event;
    routed.kind = INPUT_KIND_POINTER_ABS;
    routed.x = (x - hover.x) as i32;
    routed.y = (y - hover.y) as i32;
    routed.delta_x = 0;
    routed.delta_y = 0;
    if !ctx.subscriptions.allows(hover.pid, routed.kind) {
        return 0;
    }
    let delivered = deliver_to(hover.pid, Some(hover.window_id), &routed);
    if delivered == 0 {
        ctx.forget_pid(hover.pid);
        ctx.hover = None;
    }
    delivered
}

pub(super) fn send_leave(ctx: &mut Context, left: Hover, event: &InputEvent) {
    let pid = left.pid;
    let mut routed = *event;
    routed.kind = INPUT_KIND_POINTER_ABS;
    routed.x = -1;
    routed.y = -1;
    routed.delta_x = 0;
    routed.delta_y = 0;
    if ctx.subscriptions.allows(pid, routed.kind)
        && deliver_to(pid, Some(left.window_id), &routed) == 0
    {
        ctx.forget_pid(pid);
    }
}
