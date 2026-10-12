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

//! Receiving one NNET request, waiting at most `wait_ms` (0: until one
//! comes), and answering it.

use nonos_libc::{mk_ipc_recv_from, mk_ipc_reply};

use crate::nic::Nic;
use crate::nnet::{answer, decode, status, Request, Stats, E_INVAL, HDR_LEN};

const SERVICE_INBOX: u64 = 0;

pub(super) fn serve_one<N: Nic>(
    rx: &mut [u8],
    tx: &mut [u8],
    wait_ms: u64,
    nic: Option<&mut N>,
    stats: &mut Stats,
) {
    let mut from = 0u32;
    let n = mk_ipc_recv_from(SERVICE_INBOX, rx.as_mut_ptr(), rx.len(), wait_ms, &mut from);
    // Sender 0 is a delivery from the kernel itself, which has no one to
    // answer. Nothing here sends to this inbox, and the capsule could not
    // tell its own pid if it did: MkGetPid needs CoreExec, outside its
    // mask 0x38, so libc's mk_getpid gives 0.
    if n <= 0 || from == 0 {
        return;
    }
    let n = (n as usize).min(rx.len());
    let len = match decode(&rx[..n]) {
        Some(req) => answer(nic, stats, &req, &rx[HDR_LEN..n], tx),
        None => {
            let none = Request { op: 0, flags: 0, request_id: 0, payload_len: 0 };
            status(tx, &none, E_INVAL)
        }
    };
    let _ = mk_ipc_reply(from, tx.as_ptr(), len);
}
