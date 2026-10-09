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

//! Locking a session whose owner ended without ending it
//! (`state/context/owner_ended.rs`). It is one pid, asked about only while a
//! session is open, so every request looks before it is served: a start
//! that would be refused as busy finds the dead owner's session gone.

use nonos_libc::mk_pid_alive;

use crate::clients::{compositor, desktop_shell};
use crate::render;
use crate::state::Context;

const MSG_LOCKED: &[u8] = b"login:session_locked";

fn alive(pid: u32) -> bool {
    mk_pid_alive(pid)
}

/// Lock the session of an owner that ended, and tell the shell and redraw
/// the lock screen as the owner's own end would have. The keyring drops the
/// ended owner's keys itself, so there is no key to lock on its behalf.
pub(super) fn reap_now(ctx: &mut Context, request_id: u32) {
    if ctx.end_if_owner_ended(alive).is_none() {
        return;
    }
    let _ = desktop_shell::notify_info(ctx.desktop_shell_port, request_id, MSG_LOCKED);
    render::paint_locked(ctx);
    let _ = compositor::ping_damage(ctx.compositor_port, request_id);
}
