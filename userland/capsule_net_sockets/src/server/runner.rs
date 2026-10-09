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
use nonos_libc::mk_ipc_recv_from;

use crate::protocol::E_BAD_OP;
use crate::server::handlers;
use crate::server::handlers::recv_cap::RECV_MAX;
use crate::server::parse_req::{parse, refused, HDR_LEN};
use crate::server::respond::respond;

const SERVICE_INBOX: u64 = 0;
const BUF_LEN: usize = HDR_LEN + RECV_MAX;
/*
 * How long to wait for a request while a connect is waiting on its handshake.
 * Nothing tells this service when a handshake completes, so it looks again
 * after this much quiet; with nothing pending it waits for requests alone.
 */
const PENDING_POLL_MS: u64 = 2;

pub fn run() -> ! {
    let mut rx = vec![0u8; BUF_LEN];
    let mut tx = vec![0u8; BUF_LEN];
    loop {
        let mut sender = 0u32;
        let wait = if handlers::connects_waiting() { PENDING_POLL_MS } else { 0 };
        let n = mk_ipc_recv_from(SERVICE_INBOX, rx.as_mut_ptr(), rx.len(), wait, &mut sender);
        handlers::advance_connects(&mut tx);
        handlers::reap_if_due();
        if n <= 0 || sender == 0 {
            continue;
        }
        let (req, body) = match parse(&rx[..n as usize]) {
            Ok(parsed) => parsed,
            Err(errno) => {
                let req = refused(&rx[..n as usize]);
                let _ = respond(sender, req.op, errno, req.request_id, 0, &mut tx);
                continue;
            }
        };
        if !handlers::dispatch(sender, &req, body, &mut tx) {
            respond(sender, req.op, E_BAD_OP, req.request_id, 0, &mut tx);
        }
    }
}
