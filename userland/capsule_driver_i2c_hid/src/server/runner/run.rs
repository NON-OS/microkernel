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

use nonos_libc::{mk_exit, mk_ipc_recv_from, mk_uptime_ms, recv_ready, EXIT_ABSENT};

use super::dispatch::dispatch;
use crate::drive::pass;
use crate::protocol::{parse, received, refused, E_INVAL, HDR_LEN, IPC_PAYLOAD_MAX};
use crate::server::respond;
use crate::state::State;

const SERVICE_INBOX: u64 = 0;

/// Wait on the inbox until the next pass is due (drive/pace.rs), make that
/// pass, then serve what came. A driver with nothing left to drive leaves
/// with EXIT_ABSENT, as one with no controller does (main.rs).
pub fn run(mut state: State) -> ! {
    let mut rx = vec![0u8; HDR_LEN + IPC_PAYLOAD_MAX];
    let mut tx = vec![0u8; HDR_LEN + IPC_PAYLOAD_MAX];
    loop {
        if state.done {
            mk_exit(EXIT_ABSENT);
        }
        let wait = state.pace.inbox_wait_ms(now_ms());
        let mut sender_pid = 0u32;
        let n = mk_ipc_recv_from(SERVICE_INBOX, rx.as_mut_ptr(), rx.len(), wait, &mut sender_pid);
        // A receive that failed without waiting sleeps here, so the passes
        // below keep their pace even if the inbox is gone.
        let ready = recv_ready(n);
        pass(&mut state, now_ms());
        if !ready || sender_pid == 0 {
            continue;
        }
        let msg = received(&rx, n);
        let Some((req, body)) = parse(msg) else {
            let _ = respond::send(sender_pid, &refused(msg), E_INVAL, &[], &mut tx);
            continue;
        };
        dispatch(&mut state, sender_pid, req, body, &mut tx);
    }
}

fn now_ms() -> u64 {
    mk_uptime_ms().max(0) as u64
}
