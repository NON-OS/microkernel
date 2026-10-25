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

//! Taking one request from the service inbox and answering it.

use nonos_libc::{mk_ipc_recv_from, mk_ipc_reply};

use super::answer::answer;
use super::play::Play;

const SERVICE_INBOX: u64 = 0;

/// Whether anything came within `wait_ms`.
pub(super) fn serve_one(
    play: &mut Play,
    rx: &mut [u8],
    tx: &mut [u8],
    wait_ms: u64,
    me: u32,
) -> bool {
    let mut from = 0u32;
    let n = mk_ipc_recv_from(SERVICE_INBOX, rx.as_mut_ptr(), rx.len(), wait_ms, &mut from);
    if n <= 0 {
        return false;
    }
    // A reply that found no waiter comes back to this inbox; what this
    // capsule sent itself is dropped, never answered.
    if from != 0 && from != me {
        let n = (n as usize).min(rx.len());
        let len = answer(play, &rx[..n], tx);
        let _ = mk_ipc_reply(from, tx.as_ptr(), len);
    }
    true
}
