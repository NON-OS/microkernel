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

use crate::compositor_client::push_focus_set;
use crate::state::Context;
use crate::z_order::bottom_up;

/// Tell the compositor `pid` has focus (0 for none), which lifts its layers.
pub fn lift(ctx: &mut Context, pid: u32) {
    let rid = ctx.issue_request_id();
    let outcome = push_focus_set(ctx.compositor_port, rid, pid);
    ctx.restack.note(outcome, mk_uptime_ms());
}

/// Once a restack is owed and due, lift every process with a window showing,
/// bottom of the stack first, so the compositor ends with this order. Each
/// call is bounded by the call budget and the pass stops at the first one
/// lost, so a compositor that is gone costs one call per backoff step.
pub fn resync(ctx: &mut Context) {
    let now = mk_uptime_ms();
    if !ctx.restack.due(now) {
        return;
    }
    let port = ctx.compositor_port;
    let mut rid = ctx.next_request_id;
    let mut lost = false;
    bottom_up(&ctx.windows, |pid| {
        let id = rid;
        rid = rid.wrapping_add(1).max(1);
        lost = !push_focus_set(port, id, pid).delivered();
        !lost
    });
    ctx.next_request_id = rid;
    if lost {
        ctx.restack.retry(mk_uptime_ms());
    } else {
        ctx.restack.done();
    }
}
