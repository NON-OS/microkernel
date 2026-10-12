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
use nonos_payment_proto::{parse_header, status_reply, EINVAL};

use super::call::Call;
use super::dispatch::dispatch;
use crate::jobs::{on_frame, solve_step, Work};
use crate::meter::{now_ms, Meter};

/// Its own reply inbox: 0x1_0000_0010 was driver_hda's too, and of two
/// holders of one reply inbox, the second registered could never answer.
const KERNEL_REPLY_ENDPOINT: u64 = 0x1_0000_001B;
const MAX_MSG: usize = 4096;

pub fn run() -> ! {
    let mut buf = vec![0u8; MAX_MSG];
    let mut meter = Meter::start();
    let mut work = Work::default();
    loop {
        let mut sender = 0u32;
        let n = mk_ipc_recv_from(0, buf.as_mut_ptr(), MAX_MSG, work.wait_ms(), &mut sender);
        work.tick(now_ms());
        // A slice of an offer's proof of work between requests (v1.4.34).
        solve_step(&mut work, &mut meter);
        // A job step's answer from settle is taken here and needs no reply.
        // A length past the buffer is not a frame this loop received whole.
        let Some(rx) = usize::try_from(n).ok().and_then(|n| buf.get(..n)).filter(|_| n > 0) else {
            continue;
        };
        if on_frame(&mut work, &meter, sender, rx) {
            continue;
        }
        let resp = match parse_header(rx) {
            Some((h, body)) => {
                let call = Call { op: h.op, flags: h.flags, id: h.request_id, body, sender };
                dispatch(&mut meter, &mut work, &call)
            }
            /*
             * Not this wire's frame: answered under op and id zero. The caller
             * is blocked in its call until a reply comes, and the kernel keeps
             * its place in this service's reply queue until then.
             */
            None => status_reply(0, 0, 0, EINVAL),
        };
        let _ = mk_ipc_send(KERNEL_REPLY_ENDPOINT, resp.as_ptr(), resp.len());
    }
}
