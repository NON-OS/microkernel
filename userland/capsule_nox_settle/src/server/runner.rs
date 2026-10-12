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

use alloc::vec;

use nonos_libc::{mk_ipc_recv_from, mk_ipc_send, mk_ipc_send_to_pid, mk_uptime_ms};
use nonos_settle_proto::{parse_header, status_reply, EINVAL, FLAG_ASYNC, HDR_LEN, MAX_RAW_TX};

use super::call::Call;
use super::dispatch::dispatch;
use super::gate::flags_admitted;
use crate::drain::{drain_all, DRAIN_EVERY_MS};
use crate::settle::Settle;

/// This capsule's own reply inbox (Capsule.mk's reply endpoint).
const KERNEL_REPLY_ENDPOINT: u64 = 0x1_0000_001C;
/// The largest request: a BROADCAST of MAX_RAW_TX with its chain id.
const MAX_MSG: usize = HDR_LEN + 8 + MAX_RAW_TX;

pub fn run() -> ! {
    let mut buf = vec![0u8; MAX_MSG];
    let mut settle = Settle::start();
    let mut next_drain = mk_uptime_ms();
    loop {
        if mk_uptime_ms() >= next_drain {
            drain_all(&mut settle);
            next_drain = mk_uptime_ms().saturating_add(DRAIN_EVERY_MS as i64);
        }
        let wait = next_drain.saturating_sub(mk_uptime_ms()).max(1) as u64;
        let mut sender = 0u32;
        let n = mk_ipc_recv_from(0, buf.as_mut_ptr(), MAX_MSG, wait, &mut sender);
        if n <= 0 {
            continue;
        }
        let mut to_pid = false;
        let resp = match parse_header(buf.get(..n as usize).unwrap_or(&[])) {
            Some((h, body)) => {
                let call = Call { op: h.op, flags: h.flags, id: h.request_id, body };
                // ASYNC: the meter is not blocked in a call, so its answer goes to its inbox.
                to_pid = h.flags & FLAG_ASYNC != 0 && flags_admitted(h.flags, h.op, sender);
                dispatch(&mut settle, sender, &call)
            }
            /*
             * Not this wire's frame: answered under op and id zero, so the
             * caller blocked in its call is released.
             */
            None => status_reply(0, 0, 0, EINVAL),
        };
        let _ = match to_pid {
            true => mk_ipc_send_to_pid(sender, resp.as_ptr(), resp.len()),
            false => mk_ipc_send(KERNEL_REPLY_ENDPOINT, resp.as_ptr(), resp.len()),
        };
    }
}
