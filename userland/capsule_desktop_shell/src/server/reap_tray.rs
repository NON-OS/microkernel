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

//! Removing the tray items of clients that ended without removing them
//! (`state/tray/table.rs`): looked for at most every REAP_GAP_MS while the
//! serve loop turns, and at once when a register finds the tray full.

use core::sync::atomic::{AtomicI64, Ordering};

use nonos_libc::{mk_pid_alive, mk_uptime_ms};

use crate::compositor_client::push_damage_commit;
use crate::render::{menubar_rect, paint_chrome, sync_toast_layer};
use crate::state::Context;

const REAP_GAP_MS: i64 = 2_000;

static LAST_REAP: AtomicI64 = AtomicI64::new(i64::MIN);

fn alive(pid: u32) -> bool {
    mk_pid_alive(pid)
}

/// Remove every item whose client has ended, and repaint the menu bar as
/// that client's own remove would have.
pub(crate) fn reap_now(ctx: &mut Context) {
    if ctx.tray.remove_ended(alive) == 0 {
        return;
    }
    paint_chrome(ctx);
    let r = menubar_rect(ctx.width);
    let rid = ctx.issue_request_id();
    let _ = push_damage_commit(ctx.compositor_port, rid, r.x, r.y, r.width, r.height);
    sync_toast_layer(ctx);
}

pub(crate) fn reap_if_due(ctx: &mut Context) {
    let now = mk_uptime_ms();
    let last = LAST_REAP.load(Ordering::Relaxed);
    if last != i64::MIN && now.wrapping_sub(last) < REAP_GAP_MS {
        return;
    }
    LAST_REAP.store(now, Ordering::Relaxed);
    reap_now(ctx);
}
