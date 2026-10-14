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

//! The inbox between two bring-up attempts: every call is answered at once
//! with E_BUSY, so a caller comes back later instead of waiting out its
//! whole timeout.

use alloc::vec;

use nonos_libc::{mk_ipc_recv_from, recv_ready, Deadline};

use crate::protocol::{parse, refused, E_BUSY, E_INVAL, HDR_LEN, IPC_PAYLOAD_MAX};
use crate::server::respond;

const SERVICE_INBOX: u64 = 0;
/// The longest one receive waits, so the deadline is checked this often.
const RECV_SLICE_MS: u64 = 20;

/// Serve the inbox for `ms`, answering every request with E_BUSY.
pub(super) fn answer_busy_for(ms: u64) {
    let deadline = Deadline::after_ms(ms);
    let mut rx = vec![0u8; HDR_LEN + IPC_PAYLOAD_MAX];
    let mut tx = vec![0u8; HDR_LEN + IPC_PAYLOAD_MAX];
    while !deadline.expired() {
        let mut sender_pid = 0u32;
        let n = mk_ipc_recv_from(
            SERVICE_INBOX,
            rx.as_mut_ptr(),
            rx.len(),
            RECV_SLICE_MS.min(ms),
            &mut sender_pid,
        );
        if !recv_ready(n) || sender_pid == 0 {
            continue;
        }
        let msg = &rx[..n as usize];
        let _ = match parse(msg) {
            Some((req, _)) => respond::send(sender_pid, &req, E_BUSY, &[], &mut tx),
            None => respond::send(sender_pid, &refused(msg), E_INVAL, &[], &mut tx),
        };
    }
}
