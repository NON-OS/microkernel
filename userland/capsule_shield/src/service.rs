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

//! The service loop: one request at a time from the wallet window, one
//! reply each. Answers are immediate; long work goes to the worker.

use nonos_libc::{mk_ipc_recv_from, mk_ipc_reply, mk_service_lookup};
use shield_wire::*;

use crate::dispatch::dispatch;

const MAX_MSG: usize = MESSAGE_MAX;

/// The pid holding `name`, for shield_wire's caller check: nothing but the
/// wallet window may call the shield.
fn pid_of(name: &[u8]) -> Option<u32> {
    let (mut port, mut pid) = (0u32, 0u32);
    let rc = mk_service_lookup(name.as_ptr(), name.len(), &mut port, &mut pid);
    (rc >= 0 && pid != 0).then_some(pid)
}

pub fn run() -> ! {
    let mut buf = vec![0u8; MAX_MSG];
    loop {
        let mut sender: u32 = 0;
        let n = mk_ipc_recv_from(0, buf.as_mut_ptr(), MAX_MSG, 0, &mut sender);
        if n <= 0 {
            continue;
        }
        let used = n as usize;
        let reply = match decode_request(&buf[..used]) {
            None => encode_reply(0, STATUS_MALFORMED, ""),
            Some(req) if !from_wallet(sender, pid_of) => encode_reply(req.seq, STATUS_DENIED, ""),
            Some(req) => {
                let (status, body) = dispatch(req.op, req.body);
                encode_reply(req.seq, status, &body)
            }
        };
        // The answer goes to the window that called, matched to its pending call by the
        // kernel. It was sent to a fixed reply endpoint, 0x1_0000_0002, which is the
        // keyring's and not this service's own reply inbox: the kernel routed every answer
        // there as an ordinary send, so the wallet's call always ran out of time and the
        // shield read as still starting. Only a wallet window calls, never the kernel.
        if sender != 0 {
            let _ = mk_ipc_reply(sender, reply.as_ptr(), reply.len());
        }
        // A request can carry recovery words: nothing of it stays.
        for b in buf[..used].iter_mut() {
            // SAFETY: `b` is a valid, aligned &mut u8 into `buf`; a volatile
            // write of 0 keeps the wipe from being elided.
            unsafe { core::ptr::write_volatile(b, 0) };
        }
    }
}
