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

use nonos_libc::mk_uptime_ms;

use crate::compositor_client::query_display_info;
use crate::protocol::{
    OP_POPUP_OPEN, OP_WINDOW_MAXIMIZE, OP_WINDOW_MOVE, OP_WINDOW_OPEN, OP_WINDOW_PLACE,
    OP_WINDOW_RESIZE, OP_WINDOW_RESTORE,
};
use crate::state::Context;

/// Read the compositor's display size again when it is due, and place from
/// then on against what it answers. A read the compositor does not answer
/// keeps the size held.
pub fn refresh(ctx: &mut Context) {
    let now = mk_uptime_ms();
    if !ctx.display.due(now) {
        return;
    }
    ctx.display.read(now);
    let rid = ctx.issue_request_id();
    let Ok(d) = query_display_info(ctx.compositor_port, rid) else { return };
    let was = (ctx.display_width, ctx.display_height);
    if was == (d.width, d.height) {
        return;
    }
    (ctx.display_width, ctx.display_height) = (d.width, d.height);
    // Said once a change, so a boot's serial shows when the size moved
    // under windows already placed.
    let line = alloc::format!("[WM] display {}x{} -> {}x{}\n", was.0, was.1, d.width, d.height);
    let _ = nonos_libc::mk_debug(line.as_ptr(), line.len());
}

/// Whether a request places a window against the display, so the size it is
/// placed on must be the compositor's now.
pub fn places(op: u16) -> bool {
    matches!(
        op,
        OP_WINDOW_OPEN
            | OP_WINDOW_MOVE
            | OP_WINDOW_RESIZE
            | OP_WINDOW_PLACE
            | OP_WINDOW_MAXIMIZE
            | OP_WINDOW_RESTORE
            | OP_POPUP_OPEN
    )
}
