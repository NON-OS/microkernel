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

use nonos_libc::{mk_ipc_recv_from, mk_ipc_send};
use nonos_settle_proto::{parse_header, reply, EINVAL, HDR_LEN, MAX_RAW_TX};

use super::call::Call;
use super::dispatch::dispatch;
use crate::settle::Settle;

/// This capsule's own reply inbox (Capsule.mk's reply endpoint).
const KERNEL_REPLY_ENDPOINT: u64 = 0x1_0000_001C;
/// The largest request: a BROADCAST of MAX_RAW_TX with its chain id.
const MAX_MSG: usize = HDR_LEN + 8 + MAX_RAW_TX;

pub fn run() -> ! {
    let mut buf = vec![0u8; MAX_MSG];
    let mut settle = Settle::start();
    loop {
        let mut sender = 0u32;
        let n = mk_ipc_recv_from(0, buf.as_mut_ptr(), MAX_MSG, 0, &mut sender);
        if n <= 0 {
            continue;
        }
        let resp = match parse_header(&buf[..n as usize]) {
            Some((h, body)) => {
                let call = Call { op: h.op, flags: h.flags, id: h.request_id, body };
                dispatch(&mut settle, &call)
            }
            /*
             * Not this wire's frame: answered under op and id zero, so the
             * caller blocked in its call is released.
             */
            None => reply(0, 0, 0, EINVAL, &[]),
        };
        let _ = mk_ipc_send(KERNEL_REPLY_ENDPOINT, resp.as_ptr(), resp.len());
    }
}
