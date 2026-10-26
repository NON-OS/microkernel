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

use crate::state::{Context, Hover};

use super::hover_deliver::{deliver_local, send_leave};
use super::shell_pid::shell_pid;
use super::topmost_target::topmost_target;

const REQUERY_EVERY: u32 = 4;

pub(super) fn hover_motion(ctx: &mut Context, event: &InputEvent, x: u32, y: u32) -> u32 {
    let now = nonos_libc::mk_uptime_ms();
    if let Some(hover) = ctx.hover {
        if hover.contains(x, y) {
            if hover.fresh(now) {
                return deliver_local(ctx, hover, event, x, y);
            }
            // Still inside, but long enough that the window manager is asked
            // again: a window opened over this one, or this one moved, would
            // otherwise keep the hover here until the pointer left the rect.
            return rehover(ctx, Some(hover), event, x, y, now);
        }
        send_leave(ctx, hover.pid, event);
        ctx.hover = None;
        // The event that leaves a window may be the first into another: it
        // asks now, so the window entered never waits on the count below.
        return rehover(ctx, None, event, x, y, now);
    }
    ctx.hover_tick = ctx.hover_tick.wrapping_add(1);
    if !ctx.hover_tick.is_multiple_of(REQUERY_EVERY) {
        // Owed, and paid when the input stops (settle_hover): the place the
        // pointer comes to rest is always asked, whatever the count.
        ctx.hover_owed = Some((*event, x, y));
        return 0;
    }
    rehover(ctx, None, event, x, y, now)
}

/// Ask the window manager about a skipped motion once the router has no input
/// waiting: at most once per pause, so the cap on asks holds while the
/// pointer moves and the window it stops over still gets the hover.
pub fn settle_hover(ctx: &mut Context) {
    let Some((event, x, y)) = ctx.hover_owed.take() else { return };
    if ctx.hover.is_none() && ctx.press.is_none() {
        rehover(ctx, None, &event, x, y, nonos_libc::mk_uptime_ms());
    }
}

// Ask the window manager what is on top at (x, y) and hover it; the window
// hovered before, when another one, is told the pointer left.
fn rehover(
    ctx: &mut Context,
    before: Option<Hover>,
    event: &InputEvent,
    x: u32,
    y: u32,
    now: i64,
) -> u32 {
    ctx.hover_owed = None;
    let shell = shell_pid(ctx);
    let next = match topmost_target(ctx, x, y) {
        Some(t) if t.owner_pid != shell => Some(Hover {
            pid: t.owner_pid,
            x: t.win_x,
            y: t.win_y,
            w: t.win_w,
            h: t.win_h,
            checked_ms: now,
        }),
        _ => None,
    };
    if let Some(before) = before.filter(|b| next.map(|n| n.pid) != Some(b.pid)) {
        send_leave(ctx, before.pid, event);
    }
    ctx.hover = next;
    match next {
        Some(hover) if hover.contains(x, y) => deliver_local(ctx, hover, event, x, y),
        _ => 0,
    }
}
