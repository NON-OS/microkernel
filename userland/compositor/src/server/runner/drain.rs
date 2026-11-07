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

use nonos_libc::ipc::MK_IPC_NO_WAIT;
use nonos_libc::{mk_ipc_recv_from_at, mk_uptime_ms};

use super::breadcrumb::{request, step};
use super::dispatch::dispatch;
use super::pace::{follow_up, paced_from, Last};
use super::stall_rule::Step;
use crate::protocol::{
    parse, OP_CURSOR_UPDATE, OP_DAMAGE_COMMIT, OP_PRESENT, OP_SCENE_SUBMIT, OP_SCENE_SUBMIT_LAYER,
};
use crate::server::respond;
use crate::state::Context;

const SERVICE_INBOX: u64 = 0;
/// The first receive of a drain blocks up to one 60 Hz frame. An incoming IPC
/// (a client's damage or scene commit) resumes the receive immediately, so the
/// compositor wakes on the actual work instead of on the periodic vsync tick,
/// which is unreliable on some hardware and left frames unpresented until a
/// stray input event happened to wake the loop. The rest of the batch waits
/// only as pace.rs says, so a burst drains in a single pass and a lone event
/// is not held to the next timer tick.
const RECV_FRAME_MS: u64 = 16;
const MAX_BATCH: usize = 16;

/// The first request this drain served (op, sender), if any. `last` is
/// when the last passes served input and drew.
pub fn drain_ipc(
    ctx: &mut Context,
    rx: &mut [u8],
    tx: &mut [u8],
    last: Last,
) -> Option<(u16, u32)> {
    let mut first = None;
    for i in 0..MAX_BATCH {
        let now = mk_uptime_ms();
        let timeout = if i == 0 {
            RECV_FRAME_MS
        } else {
            follow_up(now, paced_from(last, !ctx.damage.is_empty())).unwrap_or(MK_IPC_NO_WAIT)
        };
        step(Step::Wait, now);
        let (mut sender_pid, mut queued_ms) = (0u32, 0u64);
        let (buf, len) = (rx.as_mut_ptr(), rx.len());
        let n =
            mk_ipc_recv_from_at(SERVICE_INBOX, buf, len, timeout, &mut sender_pid, &mut queued_ms);
        if n <= 0 || sender_pid == 0 {
            return first;
        }
        let (req, body) = match parse(&rx[..n as usize]) {
            Ok(parsed) => parsed,
            Err((code, req)) => {
                let _ = respond::status(sender_pid, &req, code, tx);
                continue;
            }
        };
        first = first.or(Some((req.op, sender_pid)));
        request(req.op, sender_pid, mk_uptime_ms());
        let op = req.op;
        if dispatch(ctx, sender_pid, req, body, tx).is_err() {
            return first;
        }
        // The requests that change what is on the screen start a frame's
        // latency: the client's picture was ready when it was queued.
        if matches!(
            op,
            OP_DAMAGE_COMMIT
                | OP_PRESENT
                | OP_SCENE_SUBMIT
                | OP_SCENE_SUBMIT_LAYER
                | OP_CURSOR_UPDATE
        ) {
            ctx.trace.client(queued_ms);
        }
    }
    first
}
