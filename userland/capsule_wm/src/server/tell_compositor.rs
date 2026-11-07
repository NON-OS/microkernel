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

//! Every focus_set this window manager sends goes through here, so a call the
//! compositor never took is sent again with the rest of the stack
//! (state/restack.rs), and one it answered late is not.

use nonos_libc::mk_uptime_ms;

use crate::compositor_client::push_focus_set_window;
use crate::state::Context;
use crate::window::table::PER_OWNER;
use crate::z_order::{bottom_up, chain};

/// Tell the compositor `pid`'s window `window_id` came up, and the popups
/// over it after it, so its layers draw as this window manager stacks them.
/// A `pid` of 0 is no window focused: the compositor is told so.
pub fn lift(ctx: &mut Context, pid: u32, window_id: u32) {
    let mut ids = [0u32; PER_OWNER + 1];
    let n = if pid == 0 { 0 } else { chain(&ctx.windows, pid, window_id, &mut ids) };
    if n == 0 {
        ids[0] = window_id;
    }
    for &id in ids[..n.max(1)].iter() {
        let rid = ctx.issue_request_id();
        let outcome = push_focus_set_window(ctx.compositor_port, rid, pid, id);
        ctx.restack.note(outcome, mk_uptime_ms());
    }
}

pub fn resync(ctx: &mut Context) {
    let now = mk_uptime_ms();
    if !ctx.restack.due(now) {
        return;
    }
    let port = ctx.compositor_port;
    let mut rid = ctx.next_request_id;
    let mut lost = false;
    bottom_up(&ctx.windows, |pid, window_id| {
        let id = rid;
        rid = rid.wrapping_add(1).max(1);
        lost = !push_focus_set_window(port, id, pid, window_id).delivered();
        !lost
    });
    ctx.next_request_id = rid;
    if lost {
        ctx.restack.retry(mk_uptime_ms());
    } else {
        ctx.restack.done();
    }
}
