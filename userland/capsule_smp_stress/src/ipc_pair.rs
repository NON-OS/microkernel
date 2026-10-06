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

//! IPC round trips between two threads of this capsule, each on its own
//! `proc.<tid>` inbox. Side 0 sends a sequence number and waits for it back;
//! side 1 sends back what it receives. A reply a second late lost its wake.

use crate::stats::{self, Kind};
use crate::{ipc_reply, stall, workers};
use nonos_libc::{mk_idle_ms, mk_ipc_recv, mk_ipc_send_to_pid};

pub const PAIRS: usize = 2;
/// The caller's own `proc.<pid>` inbox, for `mk_ipc_recv`.
pub const OWN_INBOX: u64 = 0;

/// `arg` is pair * 2 + side.
pub fn run(arg: usize) {
    let Some(partner) = workers::wait_ipc_tid(arg ^ 1) else { return };
    let mut buf = [0u8; 8];
    let mut seq: u64 = 0;
    while !stats::stopping() {
        if arg % 2 == 1 {
            let got = mk_ipc_recv(OWN_INBOX, buf.as_mut_ptr(), buf.len(), stall::WAIT_TIMEOUT_MS);
            if got > 0 && mk_ipc_send_to_pid(partner, buf.as_ptr(), got as usize) < 0 {
                stats::fault(Kind::Error);
            }
            continue;
        }
        seq = seq.wrapping_add(1);
        if mk_ipc_send_to_pid(partner, seq.to_le_bytes().as_ptr(), buf.len()) < 0 {
            stats::fault(Kind::Error);
            mk_idle_ms(10);
            continue;
        }
        ipc_reply::await_reply(seq, &mut buf);
    }
}
