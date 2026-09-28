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

//! The serve loop.

extern crate alloc;

use alloc::vec;
use nonos_libc::{mk_ipc_recv_from, mk_time_millis};

use crate::manager::Manager;
use crate::protocol::{E_BAD_OP, HDR_LEN, IPC_PAYLOAD_MAX};

use super::dispatch::dispatch;
use super::idle::idle;
use super::parse_req::parse;
use super::respond::respond;

const SERVICE_INBOX: u64 = 0;

const IDLE_MS: u64 = 200;

pub fn run(tcp_port: u32) -> ! {
    let mut state = Manager::new(tcp_port);
    let mut rx = vec![0u8; HDR_LEN + IPC_PAYLOAD_MAX];
    let mut tx = vec![0u8; HDR_LEN + IPC_PAYLOAD_MAX];
    loop {
        let mut sender = 0u32;
        let n = mk_ipc_recv_from(SERVICE_INBOX, rx.as_mut_ptr(), rx.len(), IDLE_MS, &mut sender);
        let now = seconds();
        if n <= 0 || sender == 0 {
            idle(&mut state, now);
            continue;
        }
        match parse(&rx[..n as usize]) {
            Ok((req, body)) => {
                let (errno, len) = dispatch(&mut state, &req, body, now, &mut tx);
                respond(sender, req.op, errno, req.request_id, len, &mut tx);
            }
            /*
             * A frame that will not parse still gets an answer. The mixnet
             * transport dropped one once and the caller waited out its whole
             * timeout for a reply that was never coming.
             */
            Err(errno) => respond(sender, E_BAD_OP, errno, 0, 0, &mut tx),
        }
    }
}

fn seconds() -> u64 {
    (mk_time_millis() / 1_000).max(0) as u64
}
