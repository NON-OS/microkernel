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

//! SESSION_LOCK (0x0008), from the shell only: body u32, 1 lock, 0 unlock,
//! 2 ask, then up to four rectangles (x, y, width, height, u32 each) where
//! the pointer is drawn while locked. Answered with the status and the state
//! after it (u32, 1 locked).
//! E_ACCES from anyone else, E_INVAL for any other body
//! (route/session_lock.rs).

use nonos_libc::{mk_ipc_reply, mk_uptime_ms};

use crate::clients::wire::lookup_pid;
use crate::protocol::{
    response_header, write_status, Request, E_ACCES, E_INVAL, HDR_LEN, STATUS_LEN,
};
use crate::route::session_lock::{accept, after, parse, unlocked_at, Word};
use crate::server::respond;
use crate::state::Context;

const SHELL: &[u8] = b"desktop_shell";

pub fn handle(ctx: &mut Context, sender_pid: u32, req: &Request, body: &[u8], tx: &mut [u8]) {
    if !accept(sender_pid, lookup_pid(SHELL)) {
        let _ = respond::status(sender_pid, req, E_ACCES, tx);
        return;
    }
    let Some((w, shown)) = parse(body) else {
        let _ = respond::status(sender_pid, req, E_INVAL, tx);
        return;
    };
    ctx.unlocked_ms = unlocked_at(ctx.session_locked, w, mk_uptime_ms(), ctx.unlocked_ms);
    ctx.session_locked = after(ctx.session_locked, w);
    if w != Word::Ask {
        ctx.session_shown = shown;
        // The pointer's visibility changes now, not on its next move.
        ctx.cursor_dirty = true;
    }
    response_header(tx, req, (STATUS_LEN + 4) as u32);
    write_status(tx, 0);
    let at = HDR_LEN + STATUS_LEN;
    tx[at..at + 4].copy_from_slice(&u32::from(ctx.session_locked).to_le_bytes());
    let _ = mk_ipc_reply(sender_pid, tx.as_ptr(), at + 4);
}
