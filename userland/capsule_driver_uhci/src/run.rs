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

//! The driver's loop: each request waited for in the kernel, dispatched,
//! and answered to its sender. A frame that does not decode is answered
//! too, refused: its caller waits in its call for one. Every second the
//! slots of clients that exited are given back (server/many/owners.rs).

use nonos_libc::{mk_ipc_recv_from, mk_ipc_reply, mk_pid_alive, recv_ready, Deadline};

use crate::server::many::serve;
use crate::server::reply::status;
use crate::setup::Host;
use crate::wire::{decode, Request, E_INVAL, HDR_LEN, REPLY_MAX, REQUEST_MAX, STATUS_LEN};

const SERVICE_INBOX: u64 = 0;
/// A receive waits at most this long, so the sweep runs while no request
/// comes too.
const SWEEP_MS: u64 = 1_000;

pub fn run(mut host: Host) -> ! {
    let mut rx = [0u8; HDR_LEN + REQUEST_MAX];
    let mut tx = [0u8; HDR_LEN + STATUS_LEN + REPLY_MAX];
    host.front.alive = |pid| mk_pid_alive(pid);
    let mut sweep_due = Deadline::after_ms(SWEEP_MS);
    loop {
        if sweep_due.expired() {
            host.front.sweep(&mut tx);
            sweep_due = Deadline::after_ms(SWEEP_MS);
        }
        let mut sender = 0u32;
        let n = mk_ipc_recv_from(SERVICE_INBOX, rx.as_mut_ptr(), rx.len(), SWEEP_MS, &mut sender);
        if !recv_ready(n) || sender == 0 {
            continue;
        }
        let msg = &rx[..(n as usize).min(rx.len())];
        let len = match decode(msg) {
            Some(req) => match msg.get(HDR_LEN..HDR_LEN + req.payload_len as usize) {
                Some(body) => serve(&mut host.front, sender, &req, body, &mut tx),
                None => status(&mut tx, &req, E_INVAL),
            },
            None => status(&mut tx, &unknown(msg), E_INVAL),
        };
        // A lost reply leaves the caller to its own timeout.
        let _ = mk_ipc_reply(sender, tx.as_ptr(), len);
    }
}

/// A refusal keeps whatever op and id the frame carried, so the caller can
/// match it.
fn unknown(msg: &[u8]) -> Request {
    let at = |i: usize| msg.get(i).copied().unwrap_or(0);
    Request {
        op: u16::from_le_bytes([at(6), at(7)]),
        flags: 0,
        request_id: u32::from_le_bytes([at(12), at(13), at(14), at(15)]),
        payload_len: 0,
    }
}
