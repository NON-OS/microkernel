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

//! The service inbox. A probe can wait thirty seconds for a medium, and no
//! request was received in that time; it now answers the inbox in its
//! pauses (`probe_pause`).

use alloc::{vec, vec::Vec};

use nonos_libc::{mk_getpid, mk_ipc_recv_from};

use super::probe_pause::reads_no_probe_state;
use crate::protocol::*;
use crate::scan::{Medium, Report};
use crate::server::dispatch::dispatch;
use crate::server::respond;
use crate::state::State;

const SERVICE_INBOX: u64 = 0;

pub struct Inbox {
    rx: Vec<u8>,
    tx: Vec<u8>,
    /// A reply that found no waiter comes back here: dropped, not answered.
    self_pid: u32,
}

impl Inbox {
    pub fn new() -> Self {
        Self {
            rx: vec![0u8; HDR_LEN + BLK_HEADER_LEN + BLK_MAX_BYTES],
            tx: vec![0u8; HDR_LEN + STATUS_LEN + BLK_MAX_BYTES],
            self_pid: mk_getpid(),
        }
    }

    /// Answer one request, waiting up to `wait_ms`; returns the receive's
    /// result. `probing` answers `E_AGAIN` to what a probe's state backs.
    pub fn serve(
        &mut self,
        wait_ms: u64,
        state: &mut State,
        seen: (&Medium, &Report),
        probing: bool,
    ) -> i64 {
        let (mut sender, rx, tx) = (0u32, &mut self.rx, &mut self.tx);
        let n = mk_ipc_recv_from(SERVICE_INBOX, rx.as_mut_ptr(), rx.len(), wait_ms, &mut sender);
        if n <= 0 || sender == self.self_pid {
            return n;
        }
        match parse(&rx[..n as usize]) {
            Some((req, _)) if probing && !reads_no_probe_state(req.op) => {
                let _ = respond::status(sender, &req, E_AGAIN, tx);
            }
            Some((req, body)) => dispatch(state, seen.0, seen.1, sender, req, body, tx),
            None => {
                let _ = respond::status(sender, &refused(&rx[..n as usize]), E_INVAL, tx);
            }
        }
        n
    }
}
