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

//! The serve loop's turn for SERVE_ONE: each frame waiting in this
//! process's inbox, believed only from the capsule's owner (owner.rs) and
//! answered by the request it came as, and no other; then the held READ.
//! A connection whose opener no longer owns the endpoint is closed.

use nonos_libc::ipc::{mk_ipc_recv_req, mk_ipc_reply_to, IpcReqMeta, MK_IPC_NO_WAIT};
use nonos_libc::mk_uptime_ms;
use nonos_serve_wire::EBADF;

use super::answer::{answer, refused};
use super::owner::{look, owns};
use super::served::SERVED;
use crate::linux::net::sock;

const OWN_INBOX: u64 = 0;
/// Frames taken in one turn; the kernel lets a caller have 8 waiting.
const BATCH: usize = 16;
/// How often the serve loop looks while a connection is open, until the
/// kernel wakes it when a frame arrives.
const POLL_MS: u64 = 20;

pub fn pump() {
    let mut served = SERVED.0.borrow_mut();
    let Some(s) = served.as_mut() else { return };
    let now = u64::try_from(mk_uptime_ms()).unwrap_or(0);
    look(s, now, None);
    for _ in 0..BATCH {
        let mut meta = IpcReqMeta::default();
        let Some(n) = next(&mut s.buf, &mut meta) else { break };
        /* No answer is owed: nothing is taken that could not be answered. */
        if meta.request == 0 {
            continue;
        }
        if !owns(&mut s.owner, s.field.name().as_bytes(), meta.sender) {
            reply(meta.request, &answer(refused(&s.buf[..n]), EBADF, &[]));
            continue;
        }
        look(s, now, Some(meta.sender));
        let frame = &s.buf[..n];
        sock::with(|t| s.server.ask(t, meta.sender, meta.request, frame, now));
    }
    sock::with(|t| s.server.settle(t, now));
    s.server.out.drain(..).for_each(|(request, frame)| reply(request, &frame));
}

/// While a connection is open the loop waits POLL_MS at most, not 250.
pub fn wake_ms() -> Option<u64> {
    SERVED.0.borrow().as_ref().and_then(|s| s.server.conn.as_ref()).map(|_| POLL_MS)
}

/// The next frame in the inbox, taken into `buf`: its length, or None.
fn next(buf: &mut [u8], meta: &mut IpcReqMeta) -> Option<usize> {
    let n = mk_ipc_recv_req(OWN_INBOX, buf.as_mut_ptr(), buf.len(), MK_IPC_NO_WAIT, meta);
    usize::try_from(n).ok()
}

fn reply(request: u64, frame: &[u8]) {
    let _ = mk_ipc_reply_to(request, frame.as_ptr(), frame.len());
}
