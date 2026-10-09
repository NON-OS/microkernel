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

use nonos_libc::mk_ipc_recv_from;

const SERVICE_INBOX: u64 = 0;
const RECV_NOWAIT: u64 = 1;

/// A message a paced wait already received, left in the first `len` bytes of
/// the receive buffer; the next drain handles it before reading the inbox.
#[derive(Clone, Copy)]
pub(super) struct Held {
    pub len: usize,
    pub sender: u32,
}

/// The held message if there is one, else the next one in the inbox without
/// waiting. The length, as `mk_ipc_recv_from` returns it; `sender` is set.
pub(super) fn next_message(held: &mut Option<Held>, rx: &mut [u8], sender: &mut u32) -> i64 {
    match held.take() {
        Some(h) => {
            *sender = h.sender;
            h.len as i64
        }
        None => mk_ipc_recv_from(SERVICE_INBOX, rx.as_mut_ptr(), rx.len(), RECV_NOWAIT, sender),
    }
}
